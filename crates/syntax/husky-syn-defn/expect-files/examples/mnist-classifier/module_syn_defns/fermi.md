Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::PropsStruct(
                    PropsStructTypeSynDefn {
                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        decl: PropsStructTypeSynDecl {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_parameters: [],
                            fields: [
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `matches`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ty_expr_idx: 5,
                                    initialization: None,
                                    variable: 1,
                                },
                                PropsFieldDeclPattern {
                                    decorators: [],
                                    visibility: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `others`,
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    ty_expr_idx: 9,
                                    initialization: None,
                                    variable: 2,
                                },
                            ],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                    10,
                                                ),
                                                opd: 1,
                                            },
                                            SynExpr::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                opd: 2,
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                opd: 6,
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 7,
                                                argument_expr_idx: 8,
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
                                                            11,
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
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `matches`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 249,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 5,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 436,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    ty_expr_idx: 9,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 249,
                                                            },
                                                        ),
                                                    ),
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 436,
                                                            },
                                                        ),
                                                    ),
                                                    regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                            },
                                            expr_idx: 9,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::Fn(
                    FnSynDefn {
                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                        decl: FnSynDecl {
                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                            template_parameters: [],
                            parenate_parameters: [
                                SpecificParameterObelisk::Regular {
                                    pattern: 1,
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ty: 4,
                                },
                                SpecificParameterObelisk::Regular {
                                    pattern: 2,
                                    variables: ArenaIdxRange(
                                        2..3,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    ty: 11,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 12,
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
                                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            SynExpr::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 1,
                                            },
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
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExpr::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
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
                                                    19,
                                                ),
                                                opd: 6,
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 3,
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
                                                    23,
                                                ),
                                                opd: 8,
                                            },
                                            SynExpr::Ritchie {
                                                ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                ritchie_kind: FnType,
                                                lpar_token: LparRegionalToken(
                                                    RegionalTokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                parameter_ty_items: [
                                                    SynCommaListItem {
                                                        expr_idx: 7,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                                light_arrow_token: Some(
                                                    LightArrowRegionalToken(
                                                        RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    ),
                                                ),
                                                return_ty_expr: Some(
                                                    9,
                                                ),
                                            },
                                            SynExpr::FunctionApplicationOrCall {
                                                function: 5,
                                                generic_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                items: [
                                                    SynCommaListItem {
                                                        expr_idx: 10,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    25,
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 4,
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
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
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
                                                        ident: `ConcaveComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
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
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `FermiMatchResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            28,
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
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `concave_components`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `templates`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
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
                                            Parameter,
                                            Parameter,
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
                                                    `concave_components`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `templates`,
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
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `concave_components`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `templates`,
                                                        pattern_symbol_idx: 2,
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
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 2,
                                                    ty_expr_idx: 11,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 11,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 12,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                25,
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
                                                                        path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::List {
                                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                                items: [],
                                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                            SynExpr::Prefix {
                                                                opr: Tilde,
                                                                opr_regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                                opd: 1,
                                                            },
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
                                                            SynExpr::ExplicitApplication {
                                                                function_expr_idx: 2,
                                                                argument_expr_idx: 3,
                                                            },
                                                            SynExpr::List {
                                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                                    14,
                                                                ),
                                                                items: [],
                                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 2,
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
                                                                    19,
                                                                ),
                                                                opd: 6,
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 3,
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
                                                                    23,
                                                                ),
                                                                opd: 8,
                                                            },
                                                            SynExpr::Ritchie {
                                                                ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                                ritchie_kind: FnType,
                                                                lpar_token: LparRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        18,
                                                                    ),
                                                                ),
                                                                parameter_ty_items: [
                                                                    SynCommaListItem {
                                                                        expr_idx: 7,
                                                                        comma_regional_token_idx: None,
                                                                    },
                                                                ],
                                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                                    21,
                                                                ),
                                                                light_arrow_token: Some(
                                                                    LightArrowRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            22,
                                                                        ),
                                                                    ),
                                                                ),
                                                                return_ty_expr: Some(
                                                                    9,
                                                                ),
                                                            },
                                                            SynExpr::FunctionApplicationOrCall {
                                                                function: 5,
                                                                generic_arguments: None,
                                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                                    16,
                                                                ),
                                                                items: [
                                                                    SynCommaListItem {
                                                                        expr_idx: 10,
                                                                        comma_regional_token_idx: None,
                                                                    },
                                                                ],
                                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                                    25,
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                item_path_expr: 4,
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
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
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
                                                                        ident: `ConcaveComponent`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            20,
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
                                                                            24,
                                                                        ),
                                                                    },
                                                                ),
                                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::num::f32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            PrincipalEntityPathExpr::Root {
                                                                path_name_token: PathNameRegionalToken::Ident(
                                                                    IdentRegionalToken {
                                                                        ident: `FermiMatchResult`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            28,
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
                                                            data: [
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_keyword_group: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `concave_components`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_keyword_group: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `templates`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            12,
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
                                                            Parameter,
                                                            Parameter,
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
                                                                    `concave_components`,
                                                                    1,
                                                                ),
                                                            ],
                                                            [
                                                                (
                                                                    `templates`,
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
                                                            data: [],
                                                        },
                                                        current_symbol_arena: Arena {
                                                            data: [
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                        ident: `concave_components`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        13,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                                        ident: `templates`,
                                                                        pattern_symbol_idx: 2,
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
                                                                    ty_expr_idx: 4,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                            (
                                                                ExplicitRegularParameter {
                                                                    pattern_expr_idx: 2,
                                                                    ty_expr_idx: 11,
                                                                },
                                                                ArenaIdxRange(
                                                                    2..3,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    roots: [
                                                        SynExprRoot {
                                                            kind: ExplicitParameterType,
                                                            expr_idx: 4,
                                                        },
                                                        SynExprRoot {
                                                            kind: ExplicitParameterType,
                                                            expr_idx: 11,
                                                        },
                                                        SynExprRoot {
                                                            kind: ReturnType,
                                                            expr_idx: 12,
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
                                                            path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::InheritedSymbol {
                                                    ident: `concave_components`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `concave_components`,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 1,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `collect_leashes`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
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
                                                        17,
                                                    ),
                                                    opd: 3,
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                                SynExpr::Prefix {
                                                    opr: Option,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    opd: 4,
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 5,
                                                    argument_expr_idx: 6,
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `templates`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        25,
                                                    ),
                                                    inherited_symbol_idx: 2,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        10,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `ilen`,
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
                                                SynExpr::Binary {
                                                    lopd: 10,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    ropd: 11,
                                                },
                                                SynExpr::InheritedSymbol {
                                                    ident: `templates`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        34,
                                                    ),
                                                    inherited_symbol_idx: 2,
                                                    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {
                                                        ident: `templates`,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        10,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 14,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        37,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        38,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `others`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        42,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `template`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        46,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 17,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        43,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `pop_with_largest_opt_f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            44,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        45,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 18,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        47,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 16,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `push`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        41,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 19,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        48,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        52,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `others`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        54,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::FunctionApplicationOrCall {
                                                    function: 21,
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        51,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 22,
                                                            comma_regional_token_idx: Some(
                                                                RegionalTokenIdx(
                                                                    53,
                                                                ),
                                                            ),
                                                        },
                                                        SynCommaListItem {
                                                            expr_idx: 23,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        55,
                                                    ),
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        3..7,
                                                    ),
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
                                                                18,
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
                                                            ident: `FermiMatchResult`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                50,
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
                                            data: [
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 3,
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                33,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 15,
                                                },
                                                SynStmt::Eval {
                                                    expr_idx: 20,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
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
                                                                4,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::Let {
                                                    let_token: LetRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    let_variables_pattern: Ok(
                                                        LetVariableObelisk {
                                                            pattern_expr_idx: 2,
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                Some(
                                                                    ColonRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            13,
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                            ty: Some(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    assign_token: Ok(
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 8,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 10,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: Some(
                                                                        11,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    frame_var_symbol_idx: 3,
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
                                                        1..3,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            49,
                                                        ),
                                                    },
                                                    result: 24,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `others`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        11,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `matches`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `template`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                32,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    Move,
                                                    None,
                                                ],
                                            },
                                            pattern_infos: [
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
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `others`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `matches`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `template`,
                                                        3,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
                                                    Mut,
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
                                                            ident: `concave_components`,
                                                        },
                                                    },
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            2,
                                                        ),
                                                        modifier: None,
                                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                                            ident: `templates`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    56,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `others`,
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
                                                                    56,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `matches`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            31,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    49,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 10,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            33,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    49,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `template`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    LetVariables {
                                                        pattern: 2,
                                                        ty: 7,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 7,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 8,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 15,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 20,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 24,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 25,
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
        SynDefn::ImplBlock(
            ImplBlockSynDecl::Type(
                TypeImplBlockSynDecl {
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::fermi`,
                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                        disambiguator: 0,
                    },
                    template_parameters: [],
                    self_ty_expr: SelfTypeObelisk {
                        expr: 1,
                    },
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                ItemSynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TypeImplBlock(
                                        TypeImplBlockSynNodePath {
                                            path: TypeImplBlockPath {
                                                module_path: `mnist_classifier::fermi`,
                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                    2,
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
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 1,
                                },
                            ],
                            has_self_lifetime: false,
                            has_self_place: false,
                        },
                    },
                },
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                                18,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        2,
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
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        allow_self_type: True,
                                                        allow_self_value: True,
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
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 37,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
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
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 13,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 14,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 16,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
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
                                                    frame_var_symbol_idx: 2,
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
                                                        1..2,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    result: 17,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
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
                                                        `norm`,
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
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
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
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
                                                                    34,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetVariables {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 18,
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
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `rel_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `rel_norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `rel_norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                                18,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        2,
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
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `rel_norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        allow_self_type: True,
                                                        allow_self_value: True,
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
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `rel_norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 38,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
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
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `rel_norm`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 13,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 14,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 15,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 16,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
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
                                                    frame_var_symbol_idx: 2,
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
                                                        1..2,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    result: 17,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
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
                                                        `norm`,
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
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
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
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
                                                                    34,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetVariables {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 16,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 18,
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
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TypeItem(
                TypeItemSynDefn::MemoizedField(
                    TypeMemoizedFieldSynDefn {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::fermi`,
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `angle_change_norm`,
                            item_kind: MemoizedField,
                        },
                        decl: TypeMemoizedFieldSynDecl {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `mnist_classifier::fermi`,
                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    disambiguator: 0,
                                },
                                ident: `angle_change_norm`,
                                item_kind: MemoizedField,
                            },
                            return_ty: Some(
                                ReturnTypeBeforeEqObelisk {
                                    expr: 1,
                                },
                            ),
                            expr: None,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath {
                                                                path: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                        2,
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
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypeItemPath {
                                                            impl_block: TypeImplBlockPath {
                                                                module_path: `mnist_classifier::fermi`,
                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change_norm`,
                                                            item_kind: MemoizedField,
                                                        },
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
                                                            TypePath(`core::num::f32`, `Extern`),
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
                                        allow_self_type: True,
                                        allow_self_value: True,
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
                                19,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: Some(
                                                        SynExprRegion {
                                                            data: SynExprRegionData {
                                                                parent: None,
                                                                path: RegionPath::Decl(
                                                                    ItemSynNodePath::ImplBlock(
                                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                                            TypeImplBlockSynNodePath {
                                                                                path: TypeImplBlockPath {
                                                                                    module_path: `mnist_classifier::fermi`,
                                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                                                        2,
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
                                                                    allow_self_type: True,
                                                                    allow_self_value: False,
                                                                    pattern_ty_constraints: [],
                                                                },
                                                                roots: [
                                                                    SynExprRoot {
                                                                        kind: SelfType,
                                                                        expr_idx: 1,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                            },
                                                        },
                                                    ),
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TypeItem(
                                                                TypeItemSynNodePath {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath {
                                                                            impl_block: TypeImplBlockPath {
                                                                                module_path: `mnist_classifier::fermi`,
                                                                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                                disambiguator: 0,
                                                                            },
                                                                            ident: `angle_change_norm`,
                                                                            item_kind: MemoizedField,
                                                                        },
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
                                                                            TypePath(`core::num::f32`, `Extern`),
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
                                                        allow_self_type: True,
                                                        allow_self_value: True,
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
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TypeItem(
                                                    TypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypeItemPath {
                                                                impl_block: TypeImplBlockPath {
                                                                    module_path: `mnist_classifier::fermi`,
                                                                    ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `angle_change_norm`,
                                                                item_kind: MemoizedField,
                                                            },
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
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::Literal(
                                                    RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    Literal::Float(
                                                        Unspecified(
                                                            UnspecifiedFloatLiteral(
                                                                Id {
                                                                    value: 39,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 3,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                SynExpr::FrameVarDecl {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    ident: `i`,
                                                    frame_var_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::MethodApplicationOrCall {
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
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                                SynExpr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    ropd: 6,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::SelfValue(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                                SynExpr::Field {
                                                    owner: 9,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        26,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `others`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
                                                        ),
                                                    },
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `i`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        29,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSynSymbolKind::FrameVariable(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::IndexOrCompositionWithList {
                                                    owner: 10,
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        28,
                                                    ),
                                                    items: [
                                                        SynCommaListItem {
                                                            expr_idx: 11,
                                                            comma_regional_token_idx: None,
                                                        },
                                                    ],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        30,
                                                    ),
                                                },
                                                SynExpr::Field {
                                                    owner: 12,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `angle_change`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 13,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        33,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `abs`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        35,
                                                    ),
                                                    items: [],
                                                    rpar_regional_token_idx: RegionalTokenIdx(
                                                        36,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::MethodApplicationOrCall {
                                                    self_argument: 8,
                                                    dot_regional_token_idx: RegionalTokenIdx(
                                                        22,
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `max`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            23,
                                                        ),
                                                    },
                                                    generic_arguments: None,
                                                    lpar_regional_token_idx: RegionalTokenIdx(
                                                        24,
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
                                                SynExpr::Binary {
                                                    lopd: 15,
                                                    opr: Assign,
                                                    opr_regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    ropd: 16,
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `norm`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        39,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                SynExpr::Block {
                                                    stmts: ArenaIdxRange(
                                                        2..5,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                            data: [
                                                SynStmt::Eval {
                                                    expr_idx: 17,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
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
                                                        RegionalEqToken(
                                                            RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: 2,
                                                },
                                                SynStmt::ForBetween {
                                                    for_token: StmtForRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    particulars: SynForBetweenParticulars {
                                                        for_between_loop_var_regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        for_between_loop_var_ident: `i`,
                                                        for_between_loop_var_expr_idx: 5,
                                                        range: Ok(
                                                            SynForBetweenRange {
                                                                initial_boundary: SynForBetweenLoopBoundary {
                                                                    bound_expr: None,
                                                                    kind: LowerClosed,
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
                                                    frame_var_symbol_idx: 2,
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
                                                        1..2,
                                                    ),
                                                },
                                                SynStmt::Return {
                                                    return_token: ReturnRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                    result: 18,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: SynPatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `norm`,
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
                                                        `norm`,
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                        access_end: Some(
                                                            RegionalTokenIdxRangeEnd(
                                                                RegionalTokenIdx(
                                                                    40,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::LetVariable {
                                                            ident: `norm`,
                                                            pattern_symbol_idx: 1,
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
                                                                    38,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSynSymbolVariant::FrameVariable {
                                                            ident: `i`,
                                                            expr_idx: 5,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    LetVariables {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    FrameVariable,
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: LetStmtType,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: LetStmtInitialValue,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: EvalExpr,
                                                expr_idx: 17,
                                            },
                                            SynExprRoot {
                                                kind: ReturnExpr,
                                                expr_idx: 18,
                                            },
                                            SynExprRoot {
                                                kind: BlockExpr,
                                                expr_idx: 19,
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