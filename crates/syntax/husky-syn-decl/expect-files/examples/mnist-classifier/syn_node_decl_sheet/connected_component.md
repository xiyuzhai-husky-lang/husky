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
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                ident: `row_start`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
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
                                                ident: `row_end`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                            ),
                                            ty: 1,
                                            initialization: None,
                                            variable: 1,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `upper_mass`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                            ),
                                            ty: 2,
                                            initialization: None,
                                            variable: 2,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `lower_mass`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                            ),
                                            ty: 3,
                                            initialization: None,
                                            variable: 3,
                                        },
                                    ],
                                    separators: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                12,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                16,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                20,
                                            ),
                                        ),
                                    ],
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    InlineRcurlRegionalToken(
                                        RegionalTokenIdx(
                                            21,
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            15,
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
                                                            19,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `row_start`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `row_end`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `upper_mass`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `lower_mass`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
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
                                                                    value: 39,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 41,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    ty_expr_idx: 1,
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
                                                                    value: 42,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                FieldVariable {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 43,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `row_start`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `row_end`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `upper_mass`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `lower_mass`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 3,
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
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                            ty: 4,
                                            initialization: None,
                                            variable: 0,
                                        },
                                    ],
                                    separators: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                12,
                                            ),
                                        ),
                                    ],
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    InlineRcurlRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 0,
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                opd: 1,
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
                                                        ident: `RawContour`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            11,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `matches`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
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
                                                                    value: 45,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `matches`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
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
                MajorItemSynNodePath::Form(
                    FormSynNodePath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: FormSynNodePath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
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
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                            ),
                                            ty: 1,
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                FormSynNodePath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
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
                                                opd: 0,
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
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                opd: 2,
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
                                                SynPatternData::Ident {
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
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `ct`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                        ident: `ct`,
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
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
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
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
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
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                    maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                ident: `mask`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 0,
                                            initialization: None,
                                            variable: 0,
                                        },
                                    ],
                                    separators: [],
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    InlineRcurlRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                            TypePath(`mnist::BinaryImage28`, `Extern`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `mask`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
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
                                                                    value: 53,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `mask`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
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
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Form(
                    FormSynNodePath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: FormSynNodePath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
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
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 0,
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
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                FormSynNodePath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `a`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
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
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                                PatternVariable::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `a`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `x`,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                        ident: `a`,
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
                                                        ident: `x`,
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
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
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
                    FormSynNodePath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                        Fn,
                    )`, (0)),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Form(
                    FormSynNodeDecl::FunctionRitchie(
                        MajorFunctionRitchieSynNodeDecl {
                            syn_node_path: FormSynNodePath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
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
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 0,
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
                                        syn_expr_idx: 3,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                FormSynNodePath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
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
                                                SynPatternData::Ident {
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
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `img`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                        ident: `img`,
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
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
                                                    },
                                                    ty: 0,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
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
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePathData {
                                        path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                                            path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                                variable_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
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
                                                `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
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
                                                        `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
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
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                                                            `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::Type(
                    TypeImplBlockSynNodeDecl {
                        syn_node_path: TypeImplBlockSynNodePath(
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
                                variable_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
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
                                        expr: 2,
                                    },
                                ),
                            ),
                            eq_token: Ok(
                                EqRegionalToken(
                                    RegionalTokenIdx(
                                        7,
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::raw_contours`, `MemoizedField`),
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
                                                path_expr_idx: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            syn_expr_idx: 2,
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::eff_holes`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_hole_ilen`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::max_row_span`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::row_span_sum`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::row_span_sum`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::row_span_sum`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::distribution`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::distribution`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::distribution`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::upper_mass`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::lower_mass`, `MemoizedField`),
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
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 0,
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
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_span_sum`, `MethodRitchie(
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
                                                SynPatternData::Ident {
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
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `k`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                        ident: `k`,
                                                        pattern_variable_idx: 0,
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
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
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
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        disambiguated_item_path: DisambiguatedItemPath {
                                            maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_right_mass_sum`, `MethodRitchie(
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
                                                    maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_right_mass_sum`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            3,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 0,
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
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Colon(
                                    EolColonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
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
                                                variable_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypeItemPath(`<mnist_classifier::connected_component::ConnectedComponent(0)>::top_k_row_right_mass_sum`, `MethodRitchie(
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
                                                SynPatternData::Ident {
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
                                                PatternVariable::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `k`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    variable_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
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
                                                        ident: `k`,
                                                        pattern_variable_idx: 0,
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
                                        ],
                                    },
                                    pattern_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
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
    ],
}
```