Ok(
    SynNodeDeclSheet {
        [salsa id]: 24,
        decls: [
            (
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Fugitive(
                        FugitiveSynNodeDecl::Fn(
                            FnSynNodeDecl {
                                syn_node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LaOrLtRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterObelisk {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                        traits: Some(
                                                            (
                                                                ColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        6,
                                                                    ),
                                                                ),
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RaOrGtRegionalToken(
                                                RegionalTokenIdx(
                                                    8,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                9,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [
                                            SpecificParameterObelisk::Regular {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        12,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                17,
                                            ),
                                        ),
                                    },
                                ),
                                light_arrow_token: Ok(
                                    None,
                                ),
                                return_ty: Ok(
                                    None,
                                ),
                                eol_colon: Ok(
                                    EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
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
                                                            path: FugitivePath(`quick_sort::quick_sort`, `Fn`),
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
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::BoxColonList {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    colon_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
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
                                                            ident: `Ord`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
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
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        10,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `arr`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                11,
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
                                                        `arr`,
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
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `T`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    TemplateTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 1,
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: Traits,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 4,
                                            },
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
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Fugitive(
                        FugitiveSynNodeDecl::Fn(
                            FnSynNodeDecl {
                                syn_node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LaOrLtRegionalToken(
                                                RegionalTokenIdx(
                                                    3,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterObelisk {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                        traits: Some(
                                                            (
                                                                ColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RaOrGtRegionalToken(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [
                                            SpecificParameterObelisk::Regular {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                            SpecificParameterObelisk::Regular {
                                                pattern: 2,
                                                variables: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                ty: 5,
                                            },
                                            SpecificParameterObelisk::Regular {
                                                pattern: 3,
                                                variables: ArenaIdxRange(
                                                    4..5,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        22,
                                                    ),
                                                ),
                                                ty: 6,
                                            },
                                        ],
                                        commas: [
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
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                24,
                                            ),
                                        ),
                                    },
                                ),
                                light_arrow_token: Ok(
                                    None,
                                ),
                                return_ty: Ok(
                                    None,
                                ),
                                eol_colon: Ok(
                                    EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                25,
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
                                                            path: FugitivePath(`quick_sort::quick_sort_aux`, `Fn`),
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
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::BoxColonList {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    colon_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                            ident: `Ord`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `isize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `isize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        9,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `arr`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `low`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `high`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    None,
                                                    None,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
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
                                                    SynPatternSymbol::Atom(
                                                        3,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `arr`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `low`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `high`,
                                                        3,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
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
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `T`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    TemplateTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 1,
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 2,
                                                        ty_expr_idx: 5,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 3,
                                                        ty_expr_idx: 6,
                                                    },
                                                    ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: Traits,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 4,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 5,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 6,
                                            },
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
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::partition`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Fugitive(
                        FugitiveSynNodeDecl::Fn(
                            FnSynNodeDecl {
                                syn_node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`quick_sort::partition`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                template_parameter_decl_list: Ok(
                                    Some(
                                        Generics {
                                            langle: LaOrLtRegionalToken(
                                                RegionalTokenIdx(
                                                    3,
                                                ),
                                            ),
                                            template_parameters: [
                                                TemplateParameterObelisk {
                                                    annotated_variance_token: None,
                                                    symbol: 1,
                                                    variant: TemplateParameterDeclPatternVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                        traits: Some(
                                                            (
                                                                ColonRegionalToken(
                                                                    RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                ),
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rangle: RaOrGtRegionalToken(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
                                        lpar: LparRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                        self_value_parameter: None,
                                        comma_after_self_parameter: None,
                                        parenate_parameters: [
                                            SpecificParameterObelisk::Regular {
                                                pattern: 1,
                                                variables: ArenaIdxRange(
                                                    2..3,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        11,
                                                    ),
                                                ),
                                                ty: 4,
                                            },
                                            SpecificParameterObelisk::Regular {
                                                pattern: 2,
                                                variables: ArenaIdxRange(
                                                    3..4,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        18,
                                                    ),
                                                ),
                                                ty: 5,
                                            },
                                            SpecificParameterObelisk::Regular {
                                                pattern: 3,
                                                variables: ArenaIdxRange(
                                                    4..5,
                                                ),
                                                colon: ColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        22,
                                                    ),
                                                ),
                                                ty: 6,
                                            },
                                        ],
                                        commas: [
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
                                        rpar: RparRegionalToken(
                                            RegionalTokenIdx(
                                                24,
                                            ),
                                        ),
                                    },
                                ),
                                light_arrow_token: Ok(
                                    Some(
                                        LightArrowRegionalToken(
                                            RegionalTokenIdx(
                                                25,
                                            ),
                                        ),
                                    ),
                                ),
                                return_ty: Ok(
                                    Some(
                                        ReturnTypeBeforeColonObelisk {
                                            expr: 7,
                                        },
                                    ),
                                ),
                                eol_colon: Ok(
                                    EolRegionalToken::Colon(
                                        EolColonRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                27,
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
                                                            path: FugitivePath(`quick_sort::partition`, `Fn`),
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
                                                            MajorItemPath::Trait(
                                                                TraitPath(`core::cmp::Ord`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::BoxColonList {
                                                    lbox_regional_token_idx: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    colon_regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    items: [],
                                                    rbox_regional_token_idx: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                },
                                                SynExpr::CurrentSymbol {
                                                    ident: `T`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                SynExpr::ExplicitApplication {
                                                    function_expr_idx: 2,
                                                    argument_expr_idx: 3,
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 3,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                SynExpr::PrincipalEntityPath {
                                                    item_path_expr: 4,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::num::isize`, `Extern`),
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
                                                            ident: `Ord`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `isize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `isize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                },
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameRegionalToken::Ident(
                                                        IdentRegionalToken {
                                                            ident: `isize`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                26,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
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
                                                        symbol_modifier_keyword_group: Some(
                                                            Mut(
                                                                MutRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        9,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        ident_token: IdentRegionalToken {
                                                            ident: `arr`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `low`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                        },
                                                    },
                                                    SynPatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `high`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_expr_contracts: ArenaMap {
                                                data: [
                                                    Move,
                                                    None,
                                                    None,
                                                ],
                                            },
                                            pattern_infos: [
                                                Parameter,
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
                                                    SynPatternSymbol::Atom(
                                                        3,
                                                    ),
                                                ],
                                            },
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `arr`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `low`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `high`,
                                                        3,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_modifiers: ArenaMap {
                                                data: [
                                                    Mut,
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
                                                        modifier: Const,
                                                        access_start: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::TemplateParameter {
                                                            syn_attrs: TemplateParameterSynAttrs {
                                                                syn_attrs: [],
                                                            },
                                                            annotated_variance_token: None,
                                                            template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `T`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        4,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: Mut,
                                                        access_start: RegionalTokenIdx(
                                                            11,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `arr`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `low`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSynSymbol {
                                                        modifier: None,
                                                        access_start: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                            ident: `high`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                (
                                                    TemplateTypeParameter,
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 1,
                                                        ty_expr_idx: 4,
                                                    },
                                                    ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 2,
                                                        ty_expr_idx: 5,
                                                    },
                                                    ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 3,
                                                        ty_expr_idx: 6,
                                                    },
                                                    ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            SynExprRoot {
                                                kind: Traits,
                                                expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 4,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 5,
                                            },
                                            SynExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 6,
                                            },
                                            SynExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 7,
                                            },
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
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Fugitive(
                        FugitiveSynNodeDecl::Fn(
                            FnSynNodeDecl {
                                syn_node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
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
                                    None,
                                ),
                                return_ty: Ok(
                                    None,
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
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`quick_sort::quick_sort_works_for_integers`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
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
                                        roots: [],
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
                        FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ItemSynNodeDecl::MajorItem(
                    MajorItemSynNodeDecl::Fugitive(
                        FugitiveSynNodeDecl::Fn(
                            FnSynNodeDecl {
                                syn_node_path: FugitiveSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                template_parameter_decl_list: Ok(
                                    None,
                                ),
                                parenate_parameter_decl_list: Ok(
                                    RitchieParameters {
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
                                    None,
                                ),
                                return_ty: Ok(
                                    None,
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
                                        path: RegionPath::Decl(
                                            ItemSynNodePath::MajorItem(
                                                MajorItemSynNodePath::Fugitive(
                                                    FugitiveSynNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`quick_sort::quick_sort_works_for_strs`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
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
                                        roots: [],
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