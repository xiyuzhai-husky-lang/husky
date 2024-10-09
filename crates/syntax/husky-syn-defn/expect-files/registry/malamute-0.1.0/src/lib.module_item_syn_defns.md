```rust
[
    (
        ItemPath(`malamute::Class`),
        None,
    ),
    (
        ItemPath(`malamute::Class::#derive(0)`),
        None,
    ),
    (
        ItemPath(`malamute::Class::Known`),
        None,
    ),
    (
        ItemPath(`malamute::Class::Unknown`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAll`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAll::#derive(0)`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAll::Yes`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAll::No`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAllResult`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAllResult::#derive(0)`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAllResult::ConfidentYes`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAllResult::ConfidentNo`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAllResult::Unconfident`),
        None,
    ),
    (
        ItemPath(`malamute::narrow_down`),
        None,
    ),
    (
        ItemPath(`malamute::narrow_down::#dep(0)`),
        None,
    ),
    (
        ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
        None,
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
        Some(
            ItemSynDefn {
                body: 1,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
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
                                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
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
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 0,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::default::Default`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Default`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::default::Default`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        4,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        8,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 6,
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
                                                                            `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                                                            TraitItemKind::AssocRitchie(
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
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        ],
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
                                            data: [
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TraitForTypeItem(
                                    TraitForTypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                                                TraitItemKind::AssocRitchie(
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
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::No`),
                                        ),
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 0,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `No`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::No`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 0,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
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
                                data: [
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Type {
                                                ident: `Label`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Constant {
                                                ident: `label`,
                                            },
                                        ),
                                    },
                                ],
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
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 0,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
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
    (
        ItemPath(`malamute::Class as core::ops::Unveil(0)`),
        None,
    ),
    (
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
        None,
    ),
    (
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
        Some(
            ItemSynDefn {
                body: 9,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
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
                                                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
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
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                ModulePath(`core`),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    ModulePath(`core::ops`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Class`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::Class`, `Enum`),
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
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Runtime(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                PolyRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        6,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 10,
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
                                                                            `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                                                            TraitItemKind::AssocRitchie(
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
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedVariable {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_variable_idx: 0,
                                                inherited_variable_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::InheritedVariable {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                inherited_variable_idx: 1,
                                                inherited_variable_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Constant {
                                                        ident: `label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 7,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAll`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    ModulePath(`core`),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        ModulePath(`core::ops`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ControlFlow`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
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
                                                        ident: `one_vs_all`,
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
                                                    `one_vs_all`,
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
                                            data: [
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                                                        ident: `one_vs_all`,
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
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 4,
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
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 8,
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
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TraitForTypeItem(
                                    TraitForTypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                                                TraitItemKind::AssocRitchie(
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
                                SynExprData::InheritedVariable {
                                    ident: `one_vs_all`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `one_vs_all`,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::Class::Known`),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Template(
                                        InheritedTemplateVariable::Constant {
                                            ident: `label`,
                                        },
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 2,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 1,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        ),
                                    ),
                                },
                                SynExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 6,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        2..3,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 0,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `core`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 2,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::Module(
                                            ModulePath(`core::ops`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 3,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ControlFlow`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::ops::ControlFlow`, `Enum`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 4,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Break`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Class`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::Class`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 6,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            18,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Known`,
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::Class::Known`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                25,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 8,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            26,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `No`,
                                            regional_token_idx: RegionalTokenIdx(
                                                27,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::No`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `core`,
                                            regional_token_idx: RegionalTokenIdx(
                                                29,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 10,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            30,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                31,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::Module(
                                            ModulePath(`core::ops`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 11,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            32,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ControlFlow`,
                                            regional_token_idx: RegionalTokenIdx(
                                                33,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::ops::ControlFlow`, `Enum`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 12,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            34,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Continue`,
                                            regional_token_idx: RegionalTokenIdx(
                                                35,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 5,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 8,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_expr: Ok(
                                        0,
                                    ),
                                    eol_with_token: Ok(
                                        EolWithRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    ),
                                    case_branches: [
                                        SynCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_syn_obelisk: Ok(
                                                CasePatternSyndicate {
                                                    syn_pattern_root: CaseSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    variables: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                },
                                            ),
                                            heavy_arrow_token: Ok(
                                                HeavyArrowRegionalToken(
                                                    RegionalTokenIdx(
                                                        8,
                                                    ),
                                                ),
                                            ),
                                            stmts: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        SynCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                            case_pattern_syn_obelisk: Ok(
                                                CasePatternSyndicate {
                                                    syn_pattern_root: CaseSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    variables: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                },
                                            ),
                                            heavy_arrow_token: Ok(
                                                HeavyArrowRegionalToken(
                                                    RegionalTokenIdx(
                                                        28,
                                                    ),
                                                ),
                                            ),
                                            stmts: Ok(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ],
                        },
                        pattern_region: SynPatternRegion {
                            pattern_arena: Arena {
                                data: [
                                    SynPatternData::UnitTypeVariant {
                                        path_expr_idx: 1,
                                        path: TypeVariantPath(`malamute::OneVsAll::Yes`),
                                    },
                                    SynPatternData::UnitTypeVariant {
                                        path_expr_idx: 9,
                                        path: TypeVariantPath(`malamute::OneVsAll::No`),
                                    },
                                ],
                            },
                            pattern_contracts: [
                                Contract::Pure,
                                Contract::Pure,
                            ],
                            pattern_variable_arena: Arena {
                                data: [],
                            },
                            pattern_variable_maps: [
                                [],
                                [],
                            ],
                            pattern_variable_modifiers: ArenaMap {
                                data: [],
                            },
                        },
                        variable_region: VariableRegionData {
                            inherited_variable_arena: Arena {
                                data: [
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Type {
                                                ident: `Label`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Constant {
                                                ident: `label`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `one_vs_all`,
                                        },
                                    },
                                ],
                            },
                            current_variable_arena: Arena {
                                data: [],
                            },
                            allow_self_type: True,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Case,
                                syn_pattern_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternRootKind::Case,
                                syn_pattern_idx: 1,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 8,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 9,
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
    (
        ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
        None,
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
        None,
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
        Some(
            ItemSynDefn {
                body: 10,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
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
                                                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
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
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                12,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                20,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `LABEL`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `LABEL`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                24,
                                                            ),
                                                            current_variable_idx: 0,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExprData::CurrentVariable {
                                                            ident: `LABEL`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                25,
                                                            ),
                                                            current_variable_idx: 1,
                                                            current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `LABEL`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            10,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                ModulePath(`core`),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    ModulePath(`core::ops`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    17,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        18,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAllResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        19,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        23,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        3,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        4,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                        trai_syn_expr_idxs: [],
                                                                    },
                                                                },
                                                            },
                                                            CurrentVariableEntry {
                                                                modifier: Compterm,
                                                                access_start: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentVariableData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        7,
                                                                                    ),
                                                                                ),
                                                                                PhanRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        8,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    data: CurrentTemplateVariableData::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `LABEL`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                10,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                pattern_roots: [],
                                                expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::PrimalTrait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 12,
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
                                                                            `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                                                            TraitItemKind::AssocRitchie(
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
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedVariable {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_variable_idx: 0,
                                                inherited_variable_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::InheritedVariable {
                                                ident: `LABEL`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                inherited_variable_idx: 1,
                                                inherited_variable_kind: InheritedVariableKind::Template(
                                                    InheritedTemplateVariable::Constant {
                                                        ident: `LABEL`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::SelfType(
                                                RegionalTokenIdx(
                                                    17,
                                                ),
                                            ),
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::FunctionApplicationOrCall {
                                                function: 7,
                                                template_arguments: None,
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                                items: [],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAllResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `core`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    ModulePath(`core`),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 1,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        13,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ops`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::Module(
                                                        ModulePath(`core::ops`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Subitem {
                                                parent: 2,
                                                colon_colon_token: ColonColonRegionalToken(
                                                    RegionalTokenIdx(
                                                        15,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentRegionalToken {
                                                        ident: `ControlFlow`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            16,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::ops::ControlFlow`, `Enum`),
                                                        ),
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
                                                        ident: `one_vs_all_result`,
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
                                                    `one_vs_all_result`,
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
                                            data: [
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedVariableEntry {
                                                    modifier: Compterm,
                                                    kind: InheritedVariableKind::Template(
                                                        InheritedTemplateVariable::Constant {
                                                            ident: `LABEL`,
                                                        },
                                                    ),
                                                },
                                            ],
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
                                                        ident: `one_vs_all_result`,
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
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 4,
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
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 8,
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
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::AssocItem(
                                AssocItemSynNodePath::TraitForTypeItem(
                                    TraitForTypeItemSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::AssocItem(
                                                AssocItemSynNodePathData::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePathData {
                                                        disambiguated_item_path: DisambiguatedItemPath {
                                                            maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                                                TraitItemKind::AssocRitchie(
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
                                SynExprData::InheritedVariable {
                                    ident: `one_vs_all_result`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `one_vs_all_result`,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 7,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                                        ),
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 1,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 2,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 13,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 15,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::No`),
                                        ),
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 4,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 5,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        37,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 21,
                                    opt_path: Some(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        ),
                                    ),
                                },
                                SynExprData::Unit {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 7,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 8,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        53,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        3..4,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAllResult`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 0,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ConfidentYes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `core`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 2,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            10,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::Module(
                                            ModulePath(`core::ops`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 3,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ControlFlow`,
                                            regional_token_idx: RegionalTokenIdx(
                                                13,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::ops::ControlFlow`, `Enum`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 4,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Break`,
                                            regional_token_idx: RegionalTokenIdx(
                                                15,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 6,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            18,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Yes`,
                                            regional_token_idx: RegionalTokenIdx(
                                                19,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAllResult`,
                                            regional_token_idx: RegionalTokenIdx(
                                                22,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 8,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            23,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ConfidentNo`,
                                            regional_token_idx: RegionalTokenIdx(
                                                24,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `core`,
                                            regional_token_idx: RegionalTokenIdx(
                                                26,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 10,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            27,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                28,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::Module(
                                            ModulePath(`core::ops`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 11,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            29,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ControlFlow`,
                                            regional_token_idx: RegionalTokenIdx(
                                                30,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::ops::ControlFlow`, `Enum`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 12,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            31,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Break`,
                                            regional_token_idx: RegionalTokenIdx(
                                                32,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAll`,
                                            regional_token_idx: RegionalTokenIdx(
                                                34,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 14,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            35,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `No`,
                                            regional_token_idx: RegionalTokenIdx(
                                                36,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAll::No`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `OneVsAllResult`,
                                            regional_token_idx: RegionalTokenIdx(
                                                39,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 16,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            40,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Unconfident`,
                                            regional_token_idx: RegionalTokenIdx(
                                                41,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `core`,
                                            regional_token_idx: RegionalTokenIdx(
                                                43,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::Module(
                                        ModulePath(`core`),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 18,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            44,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                45,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::Module(
                                            ModulePath(`core::ops`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 19,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            46,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ControlFlow`,
                                            regional_token_idx: RegionalTokenIdx(
                                                47,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::ops::ControlFlow`, `Enum`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Subitem {
                                    parent: 20,
                                    colon_colon_token: ColonColonRegionalToken(
                                        RegionalTokenIdx(
                                            48,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `Continue`,
                                            regional_token_idx: RegionalTokenIdx(
                                                49,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::TypeVariant(
                                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 3,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 6,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 9,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Match {
                                    match_token: MatchRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    match_expr: Ok(
                                        0,
                                    ),
                                    eol_with_token: Ok(
                                        EolWithRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                    ),
                                    case_branches: [
                                        SynCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    4,
                                                ),
                                            ),
                                            case_pattern_syn_obelisk: Ok(
                                                CasePatternSyndicate {
                                                    syn_pattern_root: CaseSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    variables: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                },
                                            ),
                                            heavy_arrow_token: Ok(
                                                HeavyArrowRegionalToken(
                                                    RegionalTokenIdx(
                                                        8,
                                                    ),
                                                ),
                                            ),
                                            stmts: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        SynCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    21,
                                                ),
                                            ),
                                            case_pattern_syn_obelisk: Ok(
                                                CasePatternSyndicate {
                                                    syn_pattern_root: CaseSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    variables: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                },
                                            ),
                                            heavy_arrow_token: Ok(
                                                HeavyArrowRegionalToken(
                                                    RegionalTokenIdx(
                                                        25,
                                                    ),
                                                ),
                                            ),
                                            stmts: Ok(
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        },
                                        SynCaseBranch {
                                            vertical_token: VerticalRegionalToken(
                                                RegionalTokenIdx(
                                                    38,
                                                ),
                                            ),
                                            case_pattern_syn_obelisk: Ok(
                                                CasePatternSyndicate {
                                                    syn_pattern_root: CaseSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    variables: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                },
                                            ),
                                            heavy_arrow_token: Ok(
                                                HeavyArrowRegionalToken(
                                                    RegionalTokenIdx(
                                                        42,
                                                    ),
                                                ),
                                            ),
                                            stmts: Ok(
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        },
                                    ],
                                },
                            ],
                        },
                        pattern_region: SynPatternRegion {
                            pattern_arena: Arena {
                                data: [
                                    SynPatternData::UnitTypeVariant {
                                        path_expr_idx: 1,
                                        path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
                                    },
                                    SynPatternData::UnitTypeVariant {
                                        path_expr_idx: 9,
                                        path: TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
                                    },
                                    SynPatternData::UnitTypeVariant {
                                        path_expr_idx: 17,
                                        path: TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
                                    },
                                ],
                            },
                            pattern_contracts: [
                                Contract::Pure,
                                Contract::Pure,
                                Contract::Pure,
                            ],
                            pattern_variable_arena: Arena {
                                data: [],
                            },
                            pattern_variable_maps: [
                                [],
                                [],
                                [],
                            ],
                            pattern_variable_modifiers: ArenaMap {
                                data: [],
                            },
                        },
                        variable_region: VariableRegionData {
                            inherited_variable_arena: Arena {
                                data: [
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Type {
                                                ident: `Label`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Constant {
                                                ident: `LABEL`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `one_vs_all_result`,
                                        },
                                    },
                                ],
                            },
                            current_variable_arena: Arena {
                                data: [],
                            },
                            allow_self_type: True,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Case,
                                syn_pattern_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternRootKind::Case,
                                syn_pattern_idx: 1,
                            },
                            SynPatternRoot {
                                kind: SynPatternRootKind::Case,
                                syn_pattern_idx: 2,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 3,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 9,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 10,
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
]
```