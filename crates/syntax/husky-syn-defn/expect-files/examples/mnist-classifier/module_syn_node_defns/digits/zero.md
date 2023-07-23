Ok(
    [
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 26,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            6,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeEq {
                                        expr: 0,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        8,
                                    ),
                                ),
                            ),
                            expr: None,
                            expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
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
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            5,
                        ),
                        expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `FermiMatchResult`,
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
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
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                13,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                15,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                10,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            12,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                16,
                                            ),
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `almost_closed`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Eval {
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                        expr_idx: 4,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Fn(
                    FnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 27,
                            implicit_parameter_decl_list: Ok(
                                None,
                            ),
                            parenic_parameter_decl_list: Ok(
                                SelfParameterAndExplicitParameters {
                                    lpar: LeftParenthesisToken(
                                        TokenIdx(
                                            19,
                                        ),
                                    ),
                                    self_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenic_parameters: [
                                        SpecificParameterDecl::Regular {
                                            pattern: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonToken(
                                                TokenIdx(
                                                    21,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RightParenthesisToken(
                                        TokenIdx(
                                            24,
                                        ),
                                    ),
                                },
                            ),
                            curry_token: Ok(
                                Some(
                                    CurryToken(
                                        TokenIdx(
                                            25,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeColon {
                                        expr: 3,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolToken::Colon(
                                    EolColonToken {
                                        token_idx: TokenIdx(
                                            28,
                                        ),
                                    },
                                ),
                            ),
                            expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    22,
                                                ),
                                                opd: 0,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    26,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            20,
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
                                                PatternSynSymbol::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    0,
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
                                                CurrentSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        21,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            12,
                        ),
                        expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            26,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    23,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    27,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    20,
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
                                                        PatternSynSymbol::Atom(
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
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
                                                        CurrentSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                21,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                                                30,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 0,
                                            dot_token_idx: TokenIdx(
                                                31,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    32,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                34,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Literal(
                                            TokenIdx(
                                                37,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 1,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                33,
                                            ),
                                            ropd: 2,
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                36,
                                            ),
                                            opd: 3,
                                        },
                                        SynExpr::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                35,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                39,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                40,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        },
                                        SynExpr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                38,
                                            ),
                                            opd: 8,
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                43,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 9,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                42,
                                            ),
                                            ropd: 10,
                                        },
                                        SynExpr::Block {
                                            stmts: ArenaIdxRange(
                                                0..2,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    29,
                                                ),
                                            },
                                            condition: 6,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 11,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
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
                                        kind: Condition,
                                        expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 11,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 12,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::ModuleItem(
            ModuleItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Val(
                    ValSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: ValSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                    disambiguator: 0,
                                },
                            },
                            ast_idx: 28,
                            colon_token: Ok(
                                Some(
                                    ColonToken(
                                        TokenIdx(
                                            50,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeExprBeforeEq {
                                        expr: 4,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqToken(
                                    TokenIdx(
                                        56,
                                    ),
                                ),
                            ),
                            expr: None,
                            expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                entity_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            51,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            52,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            53,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subentity {
                                                parent: 2,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        54,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Zero`,
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
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
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            90,
                        ),
                        expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 0,
                                                        argument_expr_idx: 1,
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        entity_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Zero`,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAll`,
                                                                token_idx: TokenIdx(
                                                                    51,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    52,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    53,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Subentity {
                                                        parent: 2,
                                                        scope_resolution_token: ScopeResolutionToken(
                                                            TokenIdx(
                                                                54,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentToken {
                                                                ident: `Zero`,
                                                                token_idx: TokenIdx(
                                                                    55,
                                                                ),
                                                            },
                                                        ),
                                                        path: Ok(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Zero`,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
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
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 0,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                59,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `raw_contours`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 3,
                                            dot_token_idx: TokenIdx(
                                                65,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    66,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                67,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                68,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                70,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                69,
                                            ),
                                            ropd: 5,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                76,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    77,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `n`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                81,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 9,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                80,
                                            ),
                                            ropd: 10,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                84,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    85,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                87,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 13,
                                            lbox_token_idx: TokenIdx(
                                                86,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                88,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 15,
                                            be_token_idx: TokenIdx(
                                                89,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                93,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    94,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                95,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                96,
                                            ),
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                98,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                97,
                                            ),
                                            ropd: 19,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                103,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                106,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 22,
                                            lbox_token_idx: TokenIdx(
                                                105,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                107,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 24,
                                            opr: UnwrapOrComposeWithNot,
                                            opr_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 25,
                                            dot_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                111,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                112,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 26,
                                            dot_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                115,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                116,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `c`,
                                            token_idx: TokenIdx(
                                                118,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                120,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 28,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                119,
                                            ),
                                            ropd: 29,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 7,
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
                                            entity_path_expr: 8,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::List {
                                            lbox_token_idx: TokenIdx(
                                                132,
                                            ),
                                            items: [],
                                            rbox_token_idx: TokenIdx(
                                                133,
                                            ),
                                        },
                                        SynExpr::FunctionApplicationOrCall {
                                            function: 32,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                129,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 33,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            131,
                                                        ),
                                                    ),
                                                },
                                                SynCommaListItem {
                                                    expr_idx: 34,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                134,
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 10,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_zero_match`,
                                            token_idx: TokenIdx(
                                                137,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_zero_match`,
                                            token_idx: TokenIdx(
                                                141,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 39,
                                            dot_token_idx: TokenIdx(
                                                142,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    143,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_zero_match`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 41,
                                            dot_token_idx: TokenIdx(
                                                146,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                151,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    5,
                                                ),
                                            ),
                                        ),
                                        SynExpr::FunctionCall {
                                            function: 36,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                136,
                                            ),
                                            items: [
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 38,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                140,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 40,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                144,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                RegularOrVariadic(
                                                    RegularOrVariadicCallListItem {
                                                        argument_expr_idx: 42,
                                                        separator: Comma(
                                                            TokenIdx(
                                                                148,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                                Keyed(
                                                    KeyedCallListItem {
                                                        key_token_idx: TokenIdx(
                                                            149,
                                                        ),
                                                        key: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 448,
                                                                },
                                                            ),
                                                        ),
                                                        argument_expr_idx: 43,
                                                        separator: None,
                                                    },
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                152,
                                            ),
                                        },
                                        SynExpr::Suffix {
                                            opd: 44,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                153,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_zero_match`,
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 46,
                                            dot_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    157,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                159,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 47,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ropd: 48,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 11,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 50,
                                            dot_token_idx: TokenIdx(
                                                162,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 51,
                                            dot_token_idx: TokenIdx(
                                                164,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    165,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                167,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 52,
                                            lbox_token_idx: TokenIdx(
                                                166,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 53,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                168,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 54,
                                            be_token_idx: TokenIdx(
                                                169,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 5,
                                                    variables: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 12,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 56,
                                            dot_token_idx: TokenIdx(
                                                173,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    174,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 57,
                                            dot_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    176,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                178,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 58,
                                            lbox_token_idx: TokenIdx(
                                                177,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 59,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                179,
                                            ),
                                        },
                                        SynExpr::Be {
                                            src: 60,
                                            be_token_idx: TokenIdx(
                                                180,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 6,
                                                    variables: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                            ),
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 13,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 62,
                                            dot_token_idx: TokenIdx(
                                                186,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 63,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                191,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        SynExpr::IndexOrCompositionWithList {
                                            owner: 64,
                                            lbox_token_idx: TokenIdx(
                                                190,
                                            ),
                                            items: [
                                                SynCommaListItem {
                                                    expr_idx: 65,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                192,
                                            ),
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `major_hole`,
                                            token_idx: TokenIdx(
                                                196,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 67,
                                            dot_token_idx: TokenIdx(
                                                197,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    198,
                                                ),
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `major_hole`,
                                            token_idx: TokenIdx(
                                                204,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 69,
                                            dot_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 68,
                                            dot_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    200,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                201,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                202,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 70,
                                            dot_token_idx: TokenIdx(
                                                207,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    208,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                209,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 71,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                203,
                                            ),
                                            ropd: 72,
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 14,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 74,
                                            dot_token_idx: TokenIdx(
                                                215,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    216,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 15,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExpr::Field {
                                            owner: 76,
                                            dot_token_idx: TokenIdx(
                                                223,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    224,
                                                ),
                                            },
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 75,
                                            dot_token_idx: TokenIdx(
                                                217,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    218,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                219,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                220,
                                            ),
                                        },
                                        SynExpr::MethodApplicationOrCall {
                                            self_argument: 77,
                                            dot_token_idx: TokenIdx(
                                                225,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    226,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                227,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                228,
                                            ),
                                        },
                                        SynExpr::Binary {
                                            lopd: 78,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                221,
                                            ),
                                            ropd: 79,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                232,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `b`,
                                            token_idx: TokenIdx(
                                                234,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        SynExpr::Binary {
                                            lopd: 81,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 82,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `ratio`,
                                            token_idx: TokenIdx(
                                                236,
                                            ),
                                            current_symbol_idx: 10,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 10,
                                            },
                                        },
                                        SynExpr::Literal(
                                            TokenIdx(
                                                238,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        SynExpr::Binary {
                                            lopd: 84,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ropd: 85,
                                        },
                                        SynExpr::CurrentSymbol {
                                            ident: `simp_zero_match`,
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        SynExpr::Field {
                                            owner: 87,
                                            dot_token_idx: TokenIdx(
                                                243,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    244,
                                                ),
                                            },
                                        },
                                        SynExpr::PrincipalEntityPath {
                                            entity_path_expr: 17,
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
                                                7..21,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `connected_components`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        102,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subentity {
                                            parent: 6,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    123,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        124,
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
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        245,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subentity {
                                            parent: 16,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    246,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        247,
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
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    72,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        74,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 8,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    78,
                                                ),
                                            },
                                            condition: 11,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                            },
                                            condition: 16,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            condition: 20,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    99,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                EqToken(
                                                    TokenIdx(
                                                        101,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 27,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                            },
                                            condition: 30,
                                        },
                                        SynStmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    121,
                                                ),
                                            },
                                            result: 31,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    57,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        SynStmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                },
                                                condition: Ok(
                                                    6,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                71,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..7,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 4,
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
                                                EqToken(
                                                    TokenIdx(
                                                        127,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 35,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 45,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    154,
                                                ),
                                            },
                                            condition: 49,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                            condition: 55,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    171,
                                                ),
                                            },
                                            condition: 61,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    182,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        184,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 66,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    193,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        195,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 73,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 9,
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
                                                EqToken(
                                                    TokenIdx(
                                                        213,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 80,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        231,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 83,
                                        },
                                        SynStmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    235,
                                                ),
                                            },
                                            condition: 86,
                                        },
                                        SynStmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
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
                                                        241,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 88,
                                        },
                                        SynStmt::Eval {
                                            expr_idx: 89,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `n`,
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `major_hole`,
                                                    token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `ratio`,
                                                    token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                },
                                            },
                                            PatternSynExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        240,
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
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSynSymbol::Atom(
                                                0,
                                            ),
                                            PatternSynSymbol::Atom(
                                                1,
                                            ),
                                            PatternSynSymbol::Atom(
                                                2,
                                            ),
                                            PatternSynSymbol::Atom(
                                                3,
                                            ),
                                            PatternSynSymbol::Atom(
                                                4,
                                            ),
                                            PatternSynSymbol::Atom(
                                                5,
                                            ),
                                            PatternSynSymbol::Atom(
                                                6,
                                            ),
                                            PatternSynSymbol::Atom(
                                                7,
                                            ),
                                            PatternSynSymbol::Atom(
                                                8,
                                            ),
                                            PatternSynSymbol::Atom(
                                                9,
                                            ),
                                            PatternSynSymbol::Atom(
                                                10,
                                            ),
                                            PatternSynSymbol::Atom(
                                                11,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `none`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `n`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                2,
                                            ),
                                        ],
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
                                        [
                                            (
                                                `none`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `major_hole`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `a`,
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                `b`,
                                                9,
                                            ),
                                        ],
                                        [
                                            (
                                                `ratio`,
                                                10,
                                            ),
                                        ],
                                        [
                                            (
                                                `a`,
                                                11,
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
                                        ],
                                    },
                                },
                                symbol_region: SynSymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    61,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    74,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            125,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `n`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    91,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            125,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    101,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            125,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `c`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    127,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `simp_zero_match`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    171,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    182,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    184,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `major_hole`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    195,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `a`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    213,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `b`,
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    231,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `ratio`,
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    241,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            248,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `a`,
                                                    pattern_symbol_idx: 11,
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
                                        expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 8,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 11,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 16,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 20,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 27,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 30,
                                    },
                                    SynExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 31,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 35,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 45,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 49,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 55,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 61,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 66,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 73,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 80,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 83,
                                    },
                                    SynExprRoot {
                                        kind: Condition,
                                        expr_idx: 86,
                                    },
                                    SynExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 88,
                                    },
                                    SynExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 89,
                                    },
                                    SynExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 90,
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