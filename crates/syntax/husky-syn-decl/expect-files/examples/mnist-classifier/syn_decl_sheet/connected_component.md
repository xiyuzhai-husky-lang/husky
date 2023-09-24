Ok(
    SynDeclSheet {
        [salsa id]: 26,
        decls: [
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::PropsStruct(
                            PropsStructTypeSynDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                template_parameters: [],
                                fields: [
                                    PropsFieldDeclPattern {
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
                                        ty_expr_idx: 1,
                                        initialization: None,
                                        variable: 1,
                                    },
                                    PropsFieldDeclPattern {
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
                                        ty_expr_idx: 2,
                                        initialization: None,
                                        variable: 2,
                                    },
                                    PropsFieldDeclPattern {
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
                                        ty_expr_idx: 3,
                                        initialization: None,
                                        variable: 3,
                                    },
                                    PropsFieldDeclPattern {
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
                                        ty_expr_idx: 4,
                                        initialization: None,
                                        variable: 4,
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
                                                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 4,
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
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::FieldVariable {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `row_start`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::FieldVariable {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `row_end`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::FieldVariable {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `upper_mass`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::FieldVariable {
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
                                                                        value: 244,
                                                                    },
                                                                ),
                                                            ),
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
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
                                                                        value: 245,
                                                                    },
                                                                ),
                                                            ),
                                                            regional_token_idx: RegionalTokenIdx(
                                                                9,
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
                                                                        value: 246,
                                                                    },
                                                                ),
                                                            ),
                                                            regional_token_idx: RegionalTokenIdx(
                                                                13,
                                                            ),
                                                        },
                                                        ty_expr_idx: 3,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                                (
                                                    FieldVariable {
                                                        ident_token: IdentRegionalToken {
                                                            ident: Ident(
                                                                Coword(
                                                                    Id {
                                                                        value: 247,
                                                                    },
                                                                ),
                                                            ),
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        4..5,
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
                                                                    value: 244,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 245,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 246,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            13,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 3,
                                            },
                                            SynExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 247,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                },
                                                expr_idx: 4,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::PropsStruct(
                            PropsStructTypeSynDecl {
                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                ],
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Type(
                                                    TypeSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Fn(
                            FnSynDecl {
                                path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                template_parameters: [],
                                parenate_parameters: [
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                    path_expr_idx: 2,
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
                                                        `ct`,
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
                                                            ident: `ct`,
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
                                                        syn_pattern_root: SynPatternRoot(
                                                            1,
                                                        ),
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Type(
                        TypeSynDecl::PropsStruct(
                            PropsStructTypeSynDecl {
                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                template_parameters: [],
                                fields: [
                                    PropsFieldDeclPattern {
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
                                        ty_expr_idx: 1,
                                        initialization: None,
                                        variable: 1,
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
                                                            path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                                            ident: `BinaryImage28`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                        symbol_region: SynSymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::FieldVariable {
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
                                                                        value: 255,
                                                                    },
                                                                ),
                                                            ),
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
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
                                        roots: [
                                            SynExprRoot {
                                                kind: PropsStructFieldType {
                                                    ident_token: IdentRegionalToken {
                                                        ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 255,
                                                                },
                                                            ),
                                                        ),
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
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
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Fn(
                            FnSynDecl {
                                path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                template_parameters: [],
                                parenate_parameters: [
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
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
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            2,
                                        ),
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
                                return_ty: Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 3,
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::raw_bits::r32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                PrincipalEntityPathExpr::Root {
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
                                                            ident: `a`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
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
                                                        `a`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `x`,
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
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `x`,
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
                                                        syn_pattern_root: SynPatternRoot(
                                                            1,
                                                        ),
                                                        ty_expr_idx: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        syn_pattern_root: SynPatternRoot(
                                                            2,
                                                        ),
                                                        ty_expr_idx: 2,
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
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 2,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 3,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::MajorItem(
                    MajorItemPath::Fugitive(
                        FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                    ),
                ),
                SynDecl::MajorItem(
                    MajorItemSynDecl::Fugitive(
                        FugitiveSynDecl::Fn(
                            FnSynDecl {
                                path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                template_parameters: [],
                                parenate_parameters: [
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
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
                                                            path: FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist::BinaryImage28`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
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
                                                    SynPatternExpr::Ident {
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
                                                        `img`,
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
                                                            6,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `img`,
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
                                                        syn_pattern_root: SynPatternRoot(
                                                            1,
                                                        ),
                                                        ty_expr_idx: 1,
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
                                                expr_idx: 1,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::TraitForType(
                        TraitForTypeImplBlockSynDecl {
                            path: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            template_parameters: [],
                            trai_expr: TraitObelisk {
                                expr: 1,
                            },
                            self_ty_decl: PathLeadingExpr(
                                SelfTypeObelisk {
                                    expr: 2,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::ImplBlock(
                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                TraitForTypeImplBlockSynNodePath {
                                                    path: TraitForTypeImplBlockPath {
                                                        module_path: `mnist_classifier::connected_component`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::Path(
                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::visual::Visualize`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                path_expr_idx: 2,
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
                                            PrincipalEntityPathExpr::Root {
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
                                            PrincipalEntityPathExpr::Root {
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
                                            kind: Trait,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SelfType,
                                            expr_idx: 2,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TraitForTypeItem(
                        TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TraitForTypeItem(
                        TraitForTypeItemSynDecl::MethodFn(
                            TraitForTypeMethodFnSynDecl {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_value_parameter: None,
                                parenate_parameters: [],
                                return_ty: Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 1,
                                    },
                                ),
                                syn_expr_region: SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
                                                data: SynExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        ItemSynNodePath::ImplBlock(
                                                            ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                TraitForTypeImplBlockSynNodePath {
                                                                    path: TraitForTypeImplBlockPath {
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 1,
                                                                opt_path: Some(
                                                                    PrincipalEntityPath::MajorItem(
                                                                        MajorItemPath::Trait(
                                                                            TraitPath(`core::visual::Visualize`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            SynExpr::PrincipalEntityPath {
                                                                path_expr_idx: 2,
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                            kind: Trait,
                                                            expr_idx: 1,
                                                        },
                                                        SynExprRoot {
                                                            kind: SelfType,
                                                            expr_idx: 2,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                },
                                            },
                                        ),
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TraitForTypeItemPath {
                                                                impl_block: TraitForTypeImplBlockPath {
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
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
                                                PrincipalEntityPathExpr::Root {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::ImplBlock(
                    ImplBlockPath::TypeImplBlock(
                        TypeImplBlockPath {
                            module_path: `mnist_classifier::connected_component`,
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                            disambiguator: 0,
                        },
                    ),
                ),
                SynDecl::ImplBlock(
                    ImplBlockSynDecl::Type(
                        TypeImplBlockSynDecl {
                            path: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                                        module_path: `mnist_classifier::connected_component`,
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::PrincipalEntityPath {
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
                                            PrincipalEntityPathExpr::Root {
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
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `raw_contours`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `raw_contours`,
                                    item_kind: MemoizedField,
                                },
                                return_ty: Some(
                                    ReturnTypeBeforeEqObelisk {
                                        expr: 3,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `raw_contours`,
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
                                                SynExpr::List {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        4,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 1,
                                                    argument_expr_idx: 2,
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
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
                                                expr_idx: 3,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `eff_holes`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `eff_holes`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `eff_holes`,
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
                                                    path_expr_idx: 1,
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
                                                PrincipalEntityPathExpr::Root {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_hole_ilen`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_hole_ilen`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `max_hole_ilen`,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `max_row_span`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `max_row_span`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `max_row_span`,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `row_span_sum`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `row_span_sum`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `row_span_sum`,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `distribution`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `distribution`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `distribution`,
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
                                                    path_expr_idx: 1,
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
                                                PrincipalEntityPathExpr::Root {
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `upper_mass`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `upper_mass`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `upper_mass`,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `lower_mass`,
                            item_kind: MemoizedField,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MemoizedField(
                            TypeMemoizedFieldSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `lower_mass`,
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `lower_mass`,
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
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `top_k_row_span_sum`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `top_k_row_span_sum`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_value_parameter: None,
                                parenate_parameters: [
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
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
                                ],
                                return_ty: Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 2,
                                    },
                                ),
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `top_k_row_span_sum`,
                                                                item_kind: MethodFn,
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
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
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
                                                PrincipalEntityPathExpr::Root {
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
                                                    SynPatternExpr::Ident {
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
                                                        `k`,
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
                                                            ident: `k`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitRegularParameter {
                                                        syn_pattern_root: SynPatternRoot(
                                                            1,
                                                        ),
                                                        ty_expr_idx: 1,
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
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                ItemPath::AssociatedItem(
                    AssociatedItemPath::TypeItem(
                        TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::connected_component`,
                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `top_k_row_right_mass_sum`,
                            item_kind: MethodFn,
                        },
                    ),
                ),
                SynDecl::AssociatedItem(
                    AssociatedItemSynDecl::TypeItem(
                        TypeItemSynDecl::MethodFn(
                            TypeMethodFnSynDecl {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `mnist_classifier::connected_component`,
                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                        disambiguator: 0,
                                    },
                                    ident: `top_k_row_right_mass_sum`,
                                    item_kind: MethodFn,
                                },
                                template_parameters: [],
                                self_value_parameter: None,
                                parenate_parameters: [
                                    SpecificParameterObelisk::Regular {
                                        syn_pattern_root: SynPatternRoot(
                                            1,
                                        ),
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
                                ],
                                return_ty: Some(
                                    ReturnTypeBeforeColonObelisk {
                                        expr: 2,
                                    },
                                ),
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
                                                                        module_path: `mnist_classifier::connected_component`,
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            SynExpr::PrincipalEntityPath {
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
                                                            PrincipalEntityPathExpr::Root {
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
                                                                    module_path: `mnist_classifier::connected_component`,
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    disambiguator: 0,
                                                                },
                                                                ident: `top_k_row_right_mass_sum`,
                                                                item_kind: MethodFn,
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
                                                    path_expr_idx: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    path_expr_idx: 2,
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
                                                PrincipalEntityPathExpr::Root {
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
                                                    SynPatternExpr::Ident {
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
                                                        `k`,
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
                                                            ident: `k`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [
                                                (
                                                    ExplicitRegularParameter {
                                                        syn_pattern_root: SynPatternRoot(
                                                            1,
                                                        ),
                                                        ty_expr_idx: 1,
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
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 2,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)