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
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        PropsStructTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    LcurlRegionalToken(
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
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 286,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty_syn_expr_idx: 2,
                                            initialization: None,
                                            variable: 1,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 262,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                            ty_syn_expr_idx: 5,
                                            initialization: None,
                                            variable: 2,
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
                                    phantom: PhantomData<husky_syn_decl::error::SynNodeDeclError>,
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    RcurlRegionalToken(
                                        RegionalTokenIdx(
                                            16,
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
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                opd: 1,
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
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `cc`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
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
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 286,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                    ty_expr_idx: 2,
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
                                                                    value: 262,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    ty_expr_idx: 5,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `cc`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 2,
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
                                            syn_expr_idx: 5,
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
                                            path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                        EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                    syn_expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 3,
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
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `row`,
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
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
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
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `j`,
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
                        },
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 3,
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
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `row`,
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
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
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
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `j`,
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
                        },
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 3,
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
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `row`,
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
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
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
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `j`,
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
                        },
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                            ty: 3,
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
                                        syn_expr_idx: 4,
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                        ident: `row_above`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_below`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `row_above`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `row_below`,
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    3,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
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
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row_above`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row_below`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `j`,
                                                        pattern_symbol_idx: 3,
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
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 3,
                                                    },
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
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
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 3,
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
                MajorItemSynNodePath::Fugitive(
                    FugitiveSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Fugitive(
                                    FugitiveSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 3,
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
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
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
                                                            TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `inward`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `inward`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `outward`,
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
                                                        ident: `inward`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `outward`,
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
                        },
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 3,
                                            },
                                            variables: ArenaIdxRange(
                                                3..4,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                            ty: 3,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 4,
                                            },
                                            variables: ArenaIdxRange(
                                                4..5,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            ty: 4,
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
                                        syn_expr_idx: 5,
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
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 5,
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_above`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `row_below`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `j`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `row_above`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `row_below`,
                                                    2,
                                                ),
                                            ],
                                            [
                                                (
                                                    `j`,
                                                    3,
                                                ),
                                            ],
                                            [
                                                (
                                                    `inward_direction`,
                                                    4,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
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
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row_above`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `row_below`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `j`,
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        17,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `inward_direction`,
                                                        pattern_symbol_idx: 4,
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
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 3,
                                                    },
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 4,
                                                    },
                                                    ty_expr_idx: 4,
                                                },
                                                ArenaIdxRange(
                                                    4..5,
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
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 3,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 4,
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
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 5,
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
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                        PropsStructTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                    LcurlRegionalToken(
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
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 327,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    5,
                                                ),
                                            ),
                                            ty_syn_expr_idx: 1,
                                            initialization: None,
                                            variable: 1,
                                        },
                                        PropsFieldSyndicate {
                                            decorators: [],
                                            visibility: None,
                                            ident_token: IdentRegionalToken {
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 328,
                                                        },
                                                    ),
                                                ),
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty_syn_expr_idx: 2,
                                            initialization: None,
                                            variable: 2,
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
                                    phantom: PhantomData<husky_syn_decl::error::SynNodeDeclError>,
                                },
                            ),
                            rcurl: Ok(
                                PropsStructRcurlRegionalToken(
                                    RcurlRegionalToken(
                                        RegionalTokenIdx(
                                            12,
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
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `prev1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::FieldVariable {
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
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 327,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
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
                                                                    value: 328,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::PropsStructFieldType {
                                                ident_token: IdentRegionalToken {
                                                    ident: `prev1`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                },
                                            },
                                            syn_expr_idx: 1,
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
                                            syn_expr_idx: 2,
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                                            ty: 3,
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
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                                                    6,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    7,
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
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
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
                                                    `points`,
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
                                                        ident: `points`,
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
                                                    ty_expr_idx: 3,
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
                                            syn_expr_idx: 3,
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
                    ),
                ),
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
                                            path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                MajorItemSynNodeDecl::Fugitive(
                    FugitiveSynNodeDecl::FunctionFn(
                        FnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                                            4,
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
                                                    6,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 5,
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
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Fugitive(
                                                                FugitiveSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                                                opd: 1,
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
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExpr::Ident {
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
                                                        6,
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
                                            syn_expr_idx: 5,
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
                                                module_path: `mnist_classifier::raw_contour`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::TraitForType(
                    TraitForTypeImplBlockSynNodeDecl {
                        syn_node_path: TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
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
                            expr: 1,
                        },
                        for_token: ConnectionForRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                3,
                            ),
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 2,
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
                                                                path: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                expr_arena: Arena {
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
                    },
                ),
            ),
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
                                                ItemPathId(
                                                    Id {
                                                        value: 395,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TraitForTypeItem(
                    TraitForTypeItemSynNodeDecl::MethodFn(
                        TraitForTypeMethodFnSynNodeDecl {
                            syn_node_path: TraitForTypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TraitForTypeItem(
                                            TraitForTypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 395,
                                                            },
                                                        ),
                                                    ),
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
                                        syn_expr_idx: 1,
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
                                                                                path: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                                        ty_sketch: TypeSketch::Path(
                                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                expr_arena: Arena {
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
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 395,
                                                                                },
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
                                                    value: 318,
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
                                                        value: 318,
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
                            expr: 1,
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
                                                                            value: 318,
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
                    },
                ),
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
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 396,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 396,
                                                            },
                                                        ),
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
                                                                                            value: 318,
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
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 396,
                                                                                },
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
                                                path_expr_idx: 1,
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
                        },
                    ),
                ),
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
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 397,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 397,
                                                            },
                                                        ),
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
                                                                                            value: 318,
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
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 397,
                                                                                },
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
                                                path_expr_idx: 1,
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
                        },
                    ),
                ),
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
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 398,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 398,
                                                            },
                                                        ),
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
                                                                                            value: 318,
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
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 398,
                                                                                },
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
                                                path_expr_idx: 1,
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
                        },
                    ),
                ),
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
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 399,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MemoizedField(
                        TypeMemoizedFieldSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 399,
                                                            },
                                                        ),
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
                                                                                            value: 318,
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
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 399,
                                                                                },
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
                        },
                    ),
                ),
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
                                            path: TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 400,
                                                    },
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
            ItemSynNodeDecl::AssociatedItem(
                AssociatedItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssociatedItem(
                                        AssociatedItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 400,
                                                            },
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
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    6,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        ParenateSynParameterData::Ordinary {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                            ),
                                            ty: 2,
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
                                        syn_expr_idx: 3,
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
                                                                                            value: 318,
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
                                                                        path: TypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 400,
                                                                                },
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
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                SynPatternExpr::Ident {
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
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `start`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `end`,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `start`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `end`,
                                                        pattern_symbol_idx: 2,
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
                        },
                    ),
                ),
            ),
        ),
    ],
}