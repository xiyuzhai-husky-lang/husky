```rust
SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::PropsStruct(
                        PropsStructSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            lcurl: Ok(
                                PropsStructLcurlRegionalToken(
                                    InlineLcurlRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                ),
                            ),
                            fields: Ok(
                                PunctuatedSmallList {
                                    elements: [
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `cc`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 1,
                                            initialization: None,
                                            variable: 0,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `points`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                            ty: 4,
                                            initialization: None,
                                            variable: 1,
                                        },
                                    ],
                                    separators: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                9,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    ],
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    InlineRcurlRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 0,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ident: `ConnectedComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
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
                                                        ident: `Point2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `points`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
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
                                                        ident: `cc`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `points`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `points`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Enum(
                        EnumSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                        data: [],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `row`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `j`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
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
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `row`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `j`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
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
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `row`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `j`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
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
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 3,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
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
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
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
                                                        ident: `Direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_above`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_below`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `row_above`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `row_below`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row_above`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row_below`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `j`,
                                                        pattern_variable_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 2,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
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
                                    pattern_to_current_variable_map: [
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
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            11,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                        ident: `Direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `inward`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `outward`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `inward`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `outward`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `inward`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `outward`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
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
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        9,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 2,
                                                },
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 3,
                                                },
                                                variables: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        17,
                                                    ),
                                                ),
                                                ty: 3,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                19,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            20,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            21,
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
                                            23,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::raw_bits::r32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
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
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 4,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
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
                                                        ident: `Direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_above`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_below`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `inward_direction`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                                PatternVariable::Atom(
                                                    3,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `row_above`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `row_below`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    `inward_direction`,
                                                    3,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row_above`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `row_below`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `j`,
                                                        pattern_variable_idx: 2,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `inward_direction`,
                                                        pattern_variable_idx: 3,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 3,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 2,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 3,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
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
                                    ],
                                },
                            },
                        },
                    ),
                ),
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
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::PropsStruct(
                        PropsStructSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            lcurl: Ok(
                                PropsStructLcurlRegionalToken(
                                    InlineLcurlRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                ),
                            ),
                            fields: Ok(
                                PunctuatedSmallList {
                                    elements: [
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `prev1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                            ),
                                            ty: 0,
                                            initialization: None,
                                            variable: 0,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `prev2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 1,
                                            initialization: None,
                                            variable: 1,
                                        },
                                    ],
                                    separators: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                    ],
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    InlineRcurlRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `prev1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `prev2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                8,
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
                                                        ident: `prev1`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                    ty_expr_idx: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `prev2`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `prev1`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `prev2`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
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
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                ty: 2,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 3,
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
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                        ident: `Point2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Point2d`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `points`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `points`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `points`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            0,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    MajorFormSynNodePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: MajorFormSynNodePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                Fn,
                            )`, (0)),
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_obelisk_list: Ok(
                                None,
                            ),
                            parenate_parameter_obelisk_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        6,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            9,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            10,
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
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                opd: 0,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                                        ident: `ConnectedComponent`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
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
                                                        ident: `RawContour`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `cc`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_contracts: [
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
                                            [
                                                (
                                                    `cc`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `cc`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            0,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
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
                                        path: TraitForTypeImplBlockPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::TraitForType(
                    TraitForTypeImplBlockSynNodeDecl {
                        syn_node_path: TraitForTypeImplBlockSynNodePath(
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
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        trai_expr: TraitSyndicate {
                            syn_expr_idx: 0,
                        },
                        for_token: ConnectionForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                3,
                            ),
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 1,
                            },
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
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
                                expr_arena: Arena {
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
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                pattern_roots: [],
                                expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::PrimalTrait,
                                        syn_expr_idx: 0,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 1,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
            ),
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::MethodFn(
                        TraitForTypeMethodRitchieSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
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
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameter_decl_list: Ok(
                                None,
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 0,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
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
                                                expr_arena: Arena {
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 1,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                        path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::Type(
                    TypeImplBlockSynNodeDecl {
                        syn_node_path: TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        self_ty_expr: SelfTypeSyndicate {
                            expr: 0,
                        },
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::ItemDecl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TypeImplBlock(
                                            TypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                            TypeImplBlockSynNodePathData {
                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                pattern_region: SynPatternRegion {
                                    pattern_arena: Arena {
                                        data: [],
                                    },
                                    pattern_contracts: [],
                                    pattern_variable_arena: Arena {
                                        data: [],
                                    },
                                    pattern_variable_maps: [],
                                    pattern_variable_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                variable_region: VariableRegionData {
                                    inherited_variable_arena: Arena {
                                        data: [],
                                    },
                                    current_variable_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                pattern_roots: [],
                                expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 0,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                pattern_to_current_variable_map: [],
                            },
                        },
                    },
                ),
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
                                            maybe_ambiguous_item_path: TypeItemPath(
                                                `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                                                TypeItemKind::MemoizedField,
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
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
                                        expr: 0,
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(
                                                                            `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                                                                            TypeItemKind::MemoizedField,
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                            maybe_ambiguous_item_path: TypeItemPath(
                                                `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                                TypeItemKind::MemoizedField,
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
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
                                        expr: 0,
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(
                                                                            `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                                                            TypeItemKind::MemoizedField,
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                            maybe_ambiguous_item_path: TypeItemPath(
                                                `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                                TypeItemKind::MemoizedField,
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
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
                                        expr: 0,
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(
                                                                            `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                                                                            TypeItemKind::MemoizedField,
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                            maybe_ambiguous_item_path: TypeItemPath(
                                                `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                                                TypeItemKind::MemoizedField,
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
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
                                        expr: 0,
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
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(
                                                                            `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                                                                            TypeItemKind::MemoizedField,
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
                                            data: [],
                                        },
                                        pattern_contracts: [],
                                        pattern_variable_arena: Arena {
                                            data: [],
                                        },
                                        pattern_variable_maps: [],
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                            maybe_ambiguous_item_path: TypeItemPath(
                                                `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                                                TypeItemKind::MethodRitchie(
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
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodRitchieSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                disambiguated_item_path: DisambiguatedItemPath {
                                                    maybe_ambiguous_item_path: TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
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
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 0,
                                                },
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        6,
                                                    ),
                                                ),
                                                ty: 0,
                                            },
                                        },
                                        ParenateParameterSyndicate {
                                            attrs: [
                                                (),
                                            ],
                                            const_constraint: None,
                                            nucleus: ParenateParameterSyndicateNucleus::Simple {
                                                syn_pattern_root: ParenateParameterSynPatternRoot {
                                                    syn_pattern_idx: 1,
                                                },
                                                variables: ArenaIdxRange(
                                                    1..2,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        10,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 2,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            15,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::ItemDecl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
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
                                                pattern_region: SynPatternRegion {
                                                    pattern_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_contracts: [],
                                                    pattern_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_variable_maps: [],
                                                    pattern_variable_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                variable_region: VariableRegionData {
                                                    inherited_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_variable_arena: Arena {
                                                        data: [],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 0,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                pattern_to_current_variable_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(
                                                                            `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                                                                            TypeItemKind::MethodRitchie(
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
                                    expr_arena: Arena {
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
                                    pattern_region: SynPatternRegion {
                                        pattern_arena: Arena {
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
                                        pattern_contracts: [
                                            Contract::Pure,
                                            Contract::Pure,
                                        ],
                                        pattern_variable_arena: Arena {
                                            data: [
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_variable_maps: [
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
                                        pattern_variable_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `start`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `end`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternRootKind::Parenate,
                                            syn_pattern_idx: 1,
                                        },
                                    ],
                                    expr_roots: [
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
                                    pattern_to_current_variable_map: [
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
                        },
                    ),
                ),
            ),
        ),
    ],
}
```