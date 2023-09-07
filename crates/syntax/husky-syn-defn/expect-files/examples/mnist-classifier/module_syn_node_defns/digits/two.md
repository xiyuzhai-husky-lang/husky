Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 51,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            7,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 1,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        9,
                                    ),
                                ),
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            8,
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
                                },
                            },
                        },
                        body: Some(
                            8,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `FermiMatchResult`,
                                                                token_idx: TokenIdx(
                                                                    8,
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
                                            lbox_token_idx: TokenIdx(
                                                14,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            16,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            18,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                20,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 1,
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                11,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 6,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                21,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        10,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        12,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `left_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        15,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `right_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        17,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `down_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        19,
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
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 52,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                RitchieParameters {
                                    lpar: LparToken(
                                        TokenIdx(
                                            24,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterObelisk::Regular {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    26,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            29,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            30,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 4,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                ),
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
                                                opr_token_idx: TokenIdx(
                                                    27,
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
                                                opr_token_idx: TokenIdx(
                                                    31,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            28,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            32,
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
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            25,
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
                                                    access_start: TokenIdx(
                                                        26,
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
                                },
                            },
                        },
                        body: Some(
                            9,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                        opr_token_idx: TokenIdx(
                                                            27,
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
                                                        opr_token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                        opd: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    28,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    32,
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
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    25,
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
                                                            access_start: TokenIdx(
                                                                26,
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
                                            token_idx: TokenIdx(
                                                37,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                38,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                40,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                41,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                43,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                44,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                47,
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
                                            opr_token_idx: TokenIdx(
                                                46,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                48,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                49,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    50,
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
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    34,
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
                                                EqToken(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    42,
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
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        35,
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
                                                access_start: TokenIdx(
                                                    36,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            51,
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
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 53,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                RitchieParameters {
                                    lpar: LparToken(
                                        TokenIdx(
                                            53,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterObelisk::Regular {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    55,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            58,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            59,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 4,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            62,
                                        ),
                                    },
                                ),
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
                                                opr_token_idx: TokenIdx(
                                                    56,
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
                                                opr_token_idx: TokenIdx(
                                                    60,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            57,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            61,
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
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            54,
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
                                                    access_start: TokenIdx(
                                                        55,
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
                                },
                            },
                        },
                        body: Some(
                            9,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                        opr_token_idx: TokenIdx(
                                                            56,
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
                                                        opr_token_idx: TokenIdx(
                                                            60,
                                                        ),
                                                        opd: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    57,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    61,
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
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    54,
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
                                                            access_start: TokenIdx(
                                                                55,
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
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    68,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                69,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                70,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                76,
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
                                            opr_token_idx: TokenIdx(
                                                75,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                78,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    79,
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
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    63,
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
                                                EqToken(
                                                    TokenIdx(
                                                        65,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    71,
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
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        64,
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
                                                access_start: TokenIdx(
                                                    65,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            80,
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
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 54,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                RitchieParameters {
                                    lpar: LparToken(
                                        TokenIdx(
                                            82,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        SpecificParameterObelisk::Regular {
                                            pattern: 1,
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    84,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparToken(
                                        TokenIdx(
                                            87,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            88,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 4,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            91,
                                        ),
                                    },
                                ),
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
                                                opr_token_idx: TokenIdx(
                                                    85,
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
                                                opr_token_idx: TokenIdx(
                                                    89,
                                                ),
                                                opd: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            86,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            90,
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
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            83,
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
                                                    access_start: TokenIdx(
                                                        84,
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
                                },
                            },
                        },
                        body: Some(
                            9,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                        opr_token_idx: TokenIdx(
                                                            85,
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
                                                        opr_token_idx: TokenIdx(
                                                            89,
                                                        ),
                                                        opd: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    86,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    90,
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
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    83,
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
                                                            access_start: TokenIdx(
                                                                84,
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
                                            token_idx: TokenIdx(
                                                95,
                                            ),
                                            inherited_symbol_idx: 1,
                                            inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 1,
                                            dot_token_idx: TokenIdx(
                                                96,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    97,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                98,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                99,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                101,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 3,
                                            dot_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                105,
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
                                            opr_token_idx: TokenIdx(
                                                104,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                106,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                107,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    108,
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
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    92,
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
                                                EqToken(
                                                    TokenIdx(
                                                        94,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 2,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    100,
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
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        93,
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
                                                access_start: TokenIdx(
                                                    94,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            109,
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
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 55,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            115,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 5,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        121,
                                    ),
                                ),
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            116,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            117,
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
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            118,
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
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        119,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Two`,
                                                        token_idx: TokenIdx(
                                                            120,
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
                                },
                            },
                        },
                        body: Some(
                            122,
                        ),
                        syn_expr_region: SynExprRegion {
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAll`,
                                                                token_idx: TokenIdx(
                                                                    116,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    117,
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
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    118,
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
                                                        scope_resolution_token: ScopeResolutionToken(
                                                            TokenIdx(
                                                                119,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentToken {
                                                                ident: `Two`,
                                                                token_idx: TokenIdx(
                                                                    120,
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
                                            be_token_idx: TokenIdx(
                                                124,
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
                                            be_token_idx: TokenIdx(
                                                128,
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
                                            be_token_idx: TokenIdx(
                                                132,
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
                                            be_token_idx: TokenIdx(
                                                136,
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
                                            be_token_idx: TokenIdx(
                                                140,
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
                                            be_token_idx: TokenIdx(
                                                144,
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
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                153,
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
                                            dot_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                161,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 17,
                                            dot_token_idx: TokenIdx(
                                                162,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                165,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 18,
                                            lbox_token_idx: TokenIdx(
                                                164,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 19,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                166,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 20,
                                            be_token_idx: TokenIdx(
                                                167,
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
                                            dot_token_idx: TokenIdx(
                                                173,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    174,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                176,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 23,
                                            lbox_token_idx: TokenIdx(
                                                175,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 24,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                177,
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
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                185,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 27,
                                            lbox_token_idx: TokenIdx(
                                                184,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 28,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                186,
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
                                            dot_token_idx: TokenIdx(
                                                191,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    192,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                194,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 31,
                                            lbox_token_idx: TokenIdx(
                                                193,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 32,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                195,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `cc_num`,
                                            token_idx: TokenIdx(
                                                197,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                199,
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
                                            opr_token_idx: TokenIdx(
                                                198,
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
                                            dot_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 38,
                                            dot_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    209,
                                                ),
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 39,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                206,
                                            ),
                                            ropd: 40,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `lower_excess`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            current_symbol_idx: 13,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 13,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                213,
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
                                            opr_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ropd: 43,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `cc_num`,
                                            token_idx: TokenIdx(
                                                215,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                217,
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
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 46,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                220,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 48,
                                            be_token_idx: TokenIdx(
                                                221,
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
                                            token_idx: TokenIdx(
                                                224,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 50,
                                            be_token_idx: TokenIdx(
                                                225,
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
                                            token_idx: TokenIdx(
                                                230,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 52,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                231,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 53,
                                            dot_token_idx: TokenIdx(
                                                232,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    233,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                238,
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
                                            token_idx: TokenIdx(
                                                235,
                                            ),
                                            current_symbol_idx: 16,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 16,
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                            opd: 55,
                                        },
                                        SynExpr::Binary {
                                            lopd: 56,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                236,
                                            ),
                                            ropd: 57,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 59,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                243,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 60,
                                            dot_token_idx: TokenIdx(
                                                244,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    245,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                246,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                247,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 61,
                                            dot_token_idx: TokenIdx(
                                                248,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle`,
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                250,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 62,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                252,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                256,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 64,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                257,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 65,
                                            dot_token_idx: TokenIdx(
                                                258,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                260,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                261,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 66,
                                            dot_token_idx: TokenIdx(
                                                262,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    263,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                267,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 68,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                268,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 69,
                                            dot_token_idx: TokenIdx(
                                                269,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    270,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                271,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                272,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 70,
                                            dot_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                278,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 72,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                279,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 73,
                                            dot_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 74,
                                            dot_token_idx: TokenIdx(
                                                282,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    283,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                284,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                285,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                289,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 76,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                290,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 77,
                                            dot_token_idx: TokenIdx(
                                                291,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    292,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 78,
                                            dot_token_idx: TokenIdx(
                                                293,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    294,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                295,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                296,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_ymax`,
                                            token_idx: TokenIdx(
                                                301,
                                            ),
                                            current_symbol_idx: 20,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 20,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_ymin`,
                                            token_idx: TokenIdx(
                                                303,
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
                                            opr_token_idx: TokenIdx(
                                                302,
                                            ),
                                            ropd: 81,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                300,
                                            ),
                                            item: 82,
                                            rpar_token_idx: TokenIdx(
                                                304,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                306,
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
                                            opr_token_idx: TokenIdx(
                                                305,
                                            ),
                                            ropd: 84,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `right_cc`,
                                            token_idx: TokenIdx(
                                                310,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 86,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                311,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 87,
                                            dot_token_idx: TokenIdx(
                                                312,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    313,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 88,
                                            dot_token_idx: TokenIdx(
                                                314,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    315,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                316,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                317,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `right_cc`,
                                            token_idx: TokenIdx(
                                                321,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 90,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                322,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 91,
                                            dot_token_idx: TokenIdx(
                                                323,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    324,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 92,
                                            dot_token_idx: TokenIdx(
                                                325,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    326,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                327,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                328,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `right_ymax`,
                                            token_idx: TokenIdx(
                                                333,
                                            ),
                                            current_symbol_idx: 23,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 23,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `right_ymin`,
                                            token_idx: TokenIdx(
                                                335,
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
                                            opr_token_idx: TokenIdx(
                                                334,
                                            ),
                                            ropd: 95,
                                        },
                                        SynExpr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                332,
                                            ),
                                            item: 96,
                                            rpar_token_idx: TokenIdx(
                                                336,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                338,
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
                                            opr_token_idx: TokenIdx(
                                                337,
                                            ),
                                            ropd: 98,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_mid_y`,
                                            token_idx: TokenIdx(
                                                340,
                                            ),
                                            current_symbol_idx: 22,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 22,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `right_mid_y`,
                                            token_idx: TokenIdx(
                                                342,
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
                                            opr_token_idx: TokenIdx(
                                                341,
                                            ),
                                            ropd: 101,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `cc_num`,
                                            token_idx: TokenIdx(
                                                344,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                346,
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
                                            opr_token_idx: TokenIdx(
                                                345,
                                            ),
                                            ropd: 104,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `left_cc`,
                                            token_idx: TokenIdx(
                                                349,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 106,
                                            be_token_idx: TokenIdx(
                                                350,
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
                                            token_idx: TokenIdx(
                                                353,
                                            ),
                                            current_symbol_idx: 11,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 11,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 108,
                                            be_token_idx: TokenIdx(
                                                354,
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
                                            token_idx: TokenIdx(
                                                357,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        SynExpr::Be {
                                            src: 110,
                                            be_token_idx: TokenIdx(
                                                358,
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
                                            token_idx: TokenIdx(
                                                361,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 112,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                362,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 113,
                                            dot_token_idx: TokenIdx(
                                                363,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    364,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 114,
                                            dot_token_idx: TokenIdx(
                                                365,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    366,
                                                ),
                                            },
                                            generic_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                367,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                368,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                370,
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
                                            opr_token_idx: TokenIdx(
                                                369,
                                            ),
                                            ropd: 116,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `down_cc`,
                                            token_idx: TokenIdx(
                                                374,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        SynExpr::Suffix {
                                            opd: 118,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                375,
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 119,
                                            dot_token_idx: TokenIdx(
                                                376,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    377,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_zero`,
                                                    token_idx: TokenIdx(
                                                        123,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_three`,
                                                    token_idx: TokenIdx(
                                                        127,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_seven`,
                                                    token_idx: TokenIdx(
                                                        131,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        135,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_nine`,
                                                    token_idx: TokenIdx(
                                                        139,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_six`,
                                                    token_idx: TokenIdx(
                                                        143,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        149,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        157,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        172,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        181,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        190,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        203,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        207,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        378,
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
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    379,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        380,
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
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                            condition: 49,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    223,
                                                ),
                                            },
                                            condition: 51,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    227,
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
                                                EqToken(
                                                    TokenIdx(
                                                        229,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 54,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    234,
                                                ),
                                            },
                                            condition: 58,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    239,
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
                                                EqToken(
                                                    TokenIdx(
                                                        241,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 63,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    253,
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
                                                EqToken(
                                                    TokenIdx(
                                                        255,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 67,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    264,
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
                                                EqToken(
                                                    TokenIdx(
                                                        266,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 71,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    275,
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
                                                EqToken(
                                                    TokenIdx(
                                                        277,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 75,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    286,
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
                                                EqToken(
                                                    TokenIdx(
                                                        288,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 79,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    297,
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
                                                EqToken(
                                                    TokenIdx(
                                                        299,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 85,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    307,
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
                                                EqToken(
                                                    TokenIdx(
                                                        309,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 89,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    318,
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
                                                EqToken(
                                                    TokenIdx(
                                                        320,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 93,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    329,
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
                                                EqToken(
                                                    TokenIdx(
                                                        331,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 99,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                            },
                                            condition: 102,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    348,
                                                ),
                                            },
                                            condition: 107,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    352,
                                                ),
                                            },
                                            condition: 109,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    356,
                                                ),
                                            },
                                            condition: 111,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    360,
                                                ),
                                            },
                                            condition: 117,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    371,
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
                                                EqToken(
                                                    TokenIdx(
                                                        373,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 120,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    122,
                                                ),
                                            },
                                            condition: 2,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            condition: 4,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                            condition: 6,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            condition: 8,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            condition: 10,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            condition: 12,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    146,
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
                                                EqToken(
                                                    TokenIdx(
                                                        148,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 14,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    154,
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
                                                EqToken(
                                                    TokenIdx(
                                                        156,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 16,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                            condition: 21,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    169,
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
                                                EqToken(
                                                    TokenIdx(
                                                        171,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 25,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    178,
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
                                                EqToken(
                                                    TokenIdx(
                                                        180,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 29,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    187,
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
                                                EqToken(
                                                    TokenIdx(
                                                        189,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 33,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            condition: 36,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    200,
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
                                                EqToken(
                                                    TokenIdx(
                                                        202,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 41,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    210,
                                                ),
                                            },
                                            condition: 44,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                },
                                                condition: Ok(
                                                    47,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                218,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        1..15,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: SynIfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        343,
                                                    ),
                                                },
                                                condition: Ok(
                                                    105,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                347,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                stmts: Ok(
                                                    ArenaIdxRange(
                                                        15..20,
                                                    ),
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
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `cc_num`,
                                                    token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `down_cc`,
                                                    token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `end_tan`,
                                                    token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `x`,
                                                    token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `y`,
                                                    token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `left_ymax`,
                                                    token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `left_ymin`,
                                                    token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `left_mid_y`,
                                                    token_idx: TokenIdx(
                                                        298,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `right_ymax`,
                                                    token_idx: TokenIdx(
                                                        308,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `right_ymin`,
                                                    token_idx: TokenIdx(
                                                        319,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `right_mid_y`,
                                                    token_idx: TokenIdx(
                                                        330,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        351,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        355,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        359,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        372,
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
                                                access_start: TokenIdx(
                                                    126,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    130,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    134,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    138,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    142,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    146,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    148,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    156,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    169,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    171,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    180,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    189,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    202,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            381,
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
                                                access_start: TokenIdx(
                                                    223,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    227,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    229,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    241,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    255,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    266,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    277,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    288,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    299,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    309,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    320,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    331,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
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
                                                access_start: TokenIdx(
                                                    352,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            378,
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
                                                access_start: TokenIdx(
                                                    356,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            378,
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
                                                access_start: TokenIdx(
                                                    360,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            378,
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
                                                access_start: TokenIdx(
                                                    373,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            378,
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
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)