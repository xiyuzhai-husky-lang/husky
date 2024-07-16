```rust
[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Form(
                MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                    Fn,
                )`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 12,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
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
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::BoxColonList {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                colon_regional_token_idx: RegionalTokenIdx(
                                                    15,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                            },
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
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
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            MutRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `arr`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
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
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
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
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
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
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
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
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
                                            syn_expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 0,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `len`,
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                    items: [],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        8,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::CurrentVariable {
                                    ident: `len`,
                                    regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        18,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 5,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        17,
                                    ),
                                    ropd: 6,
                                },
                                SynExprData::Delimitered {
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    item: 7,
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::isize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 8,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        20,
                                    ),
                                    ropd: 9,
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 2,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    12,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 10,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        0..2,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `quick_sort_aux`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `isize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
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
                            data: [
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon_token: Ok(
                                                None,
                                            ),
                                            ty: None,
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                3,
                                            ),
                                        ),
                                    ),
                                    initial_value: 1,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 11,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `len`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
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
                                        `len`,
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
                            inherited_variable_arena: Arena {
                                data: [
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Type {
                                                ident: `T`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: RefMut,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `arr`,
                                        },
                                    },
                                ],
                            },
                            current_variable_arena: Arena {
                                data: [
                                    CurrentVariableEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    23,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `len`,
                                            pattern_variable_idx: 0,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 0,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 11,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 12,
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
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Form(
                MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                    Fn,
                )`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 22,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
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
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::BoxColonList {
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
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
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
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
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
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `low`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `high`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
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
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
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
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `high`,
                                                        pattern_variable_idx: 2,
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
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 5,
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
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
                                            syn_expr_idx: 0,
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
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                        (
                                            1,
                                            2,
                                        ),
                                        (
                                            2,
                                            3,
                                        ),
                                    ],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        2,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                                SynExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 0,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                    ropd: 1,
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                MajorFormPath(`quick_sort::partition`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        13,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                                SynExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        15,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 3,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        10,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    12,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 5,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 6,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        19,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        21,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `p`,
                                    regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 11,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        24,
                                    ),
                                    ropd: 12,
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 8,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 9,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    20,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 10,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    22,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 13,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Form(
                                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                    Fn,
                                                )`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `p`,
                                    regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        33,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                                SynExprData::FunctionApplicationOrCall {
                                    function: 15,
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    30,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 19,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    34,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 20,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        36,
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
                                            ident: `partition`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::partition`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `quick_sort_aux`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `quick_sort_aux`,
                                            regional_token_idx: RegionalTokenIdx(
                                                27,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Form(
                                            MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            6,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon_token: Ok(
                                                None,
                                            ),
                                            ty: None,
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    initial_value: 7,
                                },
                                SynStmtData::Eval {
                                    expr_idx: 14,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 21,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                1,
                                            ),
                                        },
                                        condition: Ok(
                                            2,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    5,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            0..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: None,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `p`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
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
                                        `p`,
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
                            inherited_variable_arena: Arena {
                                data: [
                                    InheritedVariableEntry {
                                        modifier: Compterm,
                                        kind: InheritedVariableKind::Template(
                                            InheritedTemplateVariable::Type {
                                                ident: `T`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: RefMut,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `arr`,
                                        },
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `low`,
                                        },
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `high`,
                                        },
                                    },
                                ],
                            },
                            current_variable_arena: Arena {
                                data: [
                                    CurrentVariableEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            8,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    37,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `p`,
                                            pattern_variable_idx: 0,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 0,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 7,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 21,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 22,
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
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Form(
                MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                    Fn,
                )`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 62,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
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
                                                        MajorItemPath::Trait(
                                                            TraitPath(`core::cmp::Ord`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::BoxColonList {
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
                                            SynExprData::CurrentVariable {
                                                ident: `T`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                current_variable_idx: 0,
                                                current_variable_kind: CurrentVariableKind::TemplateParameter {
                                                    template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                4,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::isize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
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
                                            SynPrincipalEntityPathExpr::Root {
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
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::isize`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `isize`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            27,
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
                                    pattern_expr_region: SynPatternRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        RefMut(
                                                            RefRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
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
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `low`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                },
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `high`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                BorrowMut,
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
                                                PatternVariable::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `arr`,
                                                    0,
                                                ),
                                            ],
                                            [
                                                (
                                                    `low`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `high`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                RefMut,
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
                                                    modifier: Compterm,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        data: CurrentTemplateVariableData::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                            trai_syn_expr_idxs: [
                                                                0,
                                                            ],
                                                        },
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: RefMut,
                                                    access_start: RegionalTokenIdx(
                                                        12,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `arr`,
                                                        pattern_variable_idx: 0,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        19,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `low`,
                                                        pattern_variable_idx: 1,
                                                    },
                                                },
                                                CurrentVariableEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentVariableData::SimpleParenateParameter {
                                                        ident: `high`,
                                                        pattern_variable_idx: 2,
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
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 0,
                                                    },
                                                    ty: 3,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternRoot {
                                                        syn_pattern_idx: 2,
                                                    },
                                                    ty: 5,
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
                                    ],
                                    expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::TraitInConstraint,
                                            syn_expr_idx: 0,
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
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    pattern_to_current_variable_map: [
                                        (
                                            0,
                                            1,
                                        ),
                                        (
                                            1,
                                            2,
                                        ),
                                        (
                                            2,
                                            3,
                                        ),
                                    ],
                                },
                            },
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        4,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 0,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 0,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    ropd: 1,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `low`,
                                    regional_token_idx: RegionalTokenIdx(
                                        11,
                                    ),
                                    inherited_variable_idx: 2,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `low`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        13,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 3,
                                    opr: SynBinaryOpr::Closed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    ropd: 4,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `high`,
                                    regional_token_idx: RegionalTokenIdx(
                                        18,
                                    ),
                                    inherited_variable_idx: 3,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `high`,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        20,
                                    ),
                                    LiteralTokenData::Bool(
                                        True,
                                    ),
                                ),
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        22,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        24,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 8,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        23,
                                    ),
                                    ropd: 9,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        26,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        28,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 12,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        29,
                                    ),
                                    ropd: 13,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        33,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        35,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 11,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        27,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 14,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        31,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 15,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        34,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 16,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        36,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 17,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Less,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        32,
                                    ),
                                    ropd: 18,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        38,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        40,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 20,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Add,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        39,
                                    ),
                                    ropd: 21,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        41,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        43,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 23,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        42,
                                    ),
                                    ropd: 24,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        45,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        47,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        49,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        51,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 2,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 29,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        52,
                                    ),
                                    ropd: 30,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        56,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        58,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 28,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        50,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 31,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        54,
                                    ),
                                },
                                SynExprData::IndexOrCompositionWithList {
                                    owner: 32,
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        57,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 33,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        59,
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 26,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Geq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        46,
                                    ),
                                    ropd: 27,
                                },
                                SynExprData::Binary {
                                    lopd: 34,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Greater,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        55,
                                    ),
                                    ropd: 35,
                                },
                                SynExprData::Binary {
                                    lopd: 36,
                                    opr: SynBinaryOpr::ShortCircuitLogic(
                                        BinaryShortcuitLogicOpr::And,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        48,
                                    ),
                                    ropd: 37,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        61,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        63,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::Binary {
                                    lopd: 39,
                                    opr: SynBinaryOpr::AssignClosed(
                                        BinaryClosedOpr::Sub,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        62,
                                    ),
                                    ropd: 40,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        65,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        67,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::Binary {
                                    lopd: 42,
                                    opr: SynBinaryOpr::Comparison(
                                        BinaryComparisonOpr::Geq,
                                    ),
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        66,
                                    ),
                                    ropd: 43,
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        72,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        76,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 3,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 46,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        77,
                                    ),
                                    ropd: 47,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `last_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        80,
                                    ),
                                    current_variable_idx: 2,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 2,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 49,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        81,
                                    ),
                                    ropd: 50,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 45,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        73,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `swap`,
                                        regional_token_idx: RegionalTokenIdx(
                                            74,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        75,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 48,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    79,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 51,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        83,
                                    ),
                                },
                                SynExprData::InheritedVariable {
                                    ident: `arr`,
                                    regional_token_idx: RegionalTokenIdx(
                                        84,
                                    ),
                                    inherited_variable_idx: 1,
                                    inherited_variable_kind: InheritedVariableKind::Parenate {
                                        ident: `arr`,
                                    },
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        88,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 5,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 54,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        89,
                                    ),
                                    ropd: 55,
                                },
                                SynExprData::CurrentVariable {
                                    ident: `pivot`,
                                    regional_token_idx: RegionalTokenIdx(
                                        92,
                                    ),
                                    current_variable_idx: 0,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 0,
                                    },
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 6,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::num::usize`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::Binary {
                                    lopd: 57,
                                    opr: SynBinaryOpr::As,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        93,
                                    ),
                                    ropd: 58,
                                },
                                SynExprData::MethodApplicationOrCall {
                                    self_argument: 53,
                                    dot_regional_token_idx: RegionalTokenIdx(
                                        85,
                                    ),
                                    ident_token: IdentRegionalToken {
                                        ident: `swap`,
                                        regional_token_idx: RegionalTokenIdx(
                                            86,
                                        ),
                                    },
                                    template_arguments: None,
                                    lpar_regional_token_idx: RegionalTokenIdx(
                                        87,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 56,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    91,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 59,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rpar_regional_token_idx: RegionalTokenIdx(
                                        95,
                                    ),
                                },
                                SynExprData::CurrentVariable {
                                    ident: `store_index`,
                                    regional_token_idx: RegionalTokenIdx(
                                        96,
                                    ),
                                    current_variable_idx: 1,
                                    current_variable_kind: CurrentVariableKind::LetVariable {
                                        pattern_variable_idx: 1,
                                    },
                                },
                                SynExprData::Block {
                                    stmts: ArenaIdxRange(
                                        9..15,
                                    ),
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                30,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                53,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                78,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                82,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                90,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `usize`,
                                            regional_token_idx: RegionalTokenIdx(
                                                94,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::num::usize`, `Extern`),
                                        ),
                                    ),
                                },
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Eval {
                                    expr_idx: 22,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 41,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Break {
                                    break_token: BreakRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            69,
                                        ),
                                    },
                                },
                                SynStmtData::Eval {
                                    expr_idx: 52,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 10,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            25,
                                        ),
                                    },
                                    condition: Ok(
                                        19,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    37,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        0..1,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 25,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            44,
                                        ),
                                    },
                                    condition: Ok(
                                        38,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    60,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                                SynStmtData::IfElse {
                                    if_branch: SynIfBranch {
                                        if_token: IfRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                64,
                                            ),
                                        },
                                        condition: Ok(
                                            44,
                                        ),
                                        eol_colon: Ok(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    68,
                                                ),
                                            },
                                        ),
                                        stmts: ArenaIdxRange(
                                            2..3,
                                        ),
                                    },
                                    elif_branches: [],
                                    else_branch: Some(
                                        SynElseBranch {
                                            else_token: ElseRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    70,
                                                ),
                                            },
                                            eol_colon_token: Ok(
                                                EolColonRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        71,
                                                    ),
                                                },
                                            ),
                                            stmts: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                    ),
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon_token: Ok(
                                                None,
                                            ),
                                            ty: None,
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                3,
                                            ),
                                        ),
                                    ),
                                    initial_value: 2,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 1,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                10,
                                            ),
                                        ),
                                    ),
                                    initial_value: 5,
                                },
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 2,
                                            },
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
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                17,
                                            ),
                                        ),
                                    ),
                                    initial_value: 6,
                                },
                                SynStmtData::While {
                                    while_token: WhileRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                    },
                                    condition: Ok(
                                        7,
                                    ),
                                    eol_colon: Ok(
                                        EolRegionalToken::Colon(
                                            EolColonRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                            },
                                        ),
                                    ),
                                    block: ArenaIdxRange(
                                        4..9,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 60,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                                SynStmtData::Eval {
                                    expr_idx: 61,
                                    eol_semicolon: Ok(
                                        None,
                                    ),
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: None,
                                        ident_token: IdentRegionalToken {
                                            ident: `pivot`,
                                            regional_token_idx: RegionalTokenIdx(
                                                2,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `store_index`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    },
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `last_index`,
                                            regional_token_idx: RegionalTokenIdx(
                                                16,
                                            ),
                                        },
                                    },
                                ],
                            },
                            pattern_expr_contracts: ArenaMap {
                                data: [
                                    Pure,
                                    Move,
                                    Move,
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
                                    PatternVariable::Atom(
                                        2,
                                    ),
                                ],
                            },
                            pattern_symbol_maps: [
                                [
                                    (
                                        `pivot`,
                                        0,
                                    ),
                                ],
                                [
                                    (
                                        `store_index`,
                                        1,
                                    ),
                                ],
                                [
                                    (
                                        `last_index`,
                                        2,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Pure,
                                    Mut,
                                    Mut,
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
                                                ident: `T`,
                                            },
                                        ),
                                    },
                                    InheritedVariableEntry {
                                        modifier: RefMut,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `arr`,
                                        },
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `low`,
                                        },
                                    },
                                    InheritedVariableEntry {
                                        modifier: Pure,
                                        kind: InheritedVariableKind::Parenate {
                                            ident: `high`,
                                        },
                                    },
                                ],
                            },
                            current_variable_arena: Arena {
                                data: [
                                    CurrentVariableEntry {
                                        modifier: Pure,
                                        access_start: RegionalTokenIdx(
                                            3,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    97,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `pivot`,
                                            pattern_variable_idx: 0,
                                        },
                                    },
                                    CurrentVariableEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            10,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    97,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `store_index`,
                                            pattern_variable_idx: 1,
                                        },
                                    },
                                    CurrentVariableEntry {
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            17,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    97,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `last_index`,
                                            pattern_variable_idx: 2,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 0,
                            },
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 1,
                            },
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 2,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 5,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 10,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 22,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 25,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 41,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 52,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 60,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::EvalExpr,
                                syn_expr_idx: 61,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 62,
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
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Form(
                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                    Fn,
                )`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 15,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
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
                                    pattern_expr_region: SynPatternRegion {
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
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
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
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::List {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
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
                                SynExprData::ExplicitApplication {
                                    function_expr_idx: 0,
                                    argument_expr_idx: 1,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            4,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        12,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            65,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        17,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            31,
                                        ),
                                    ),
                                ),
                                SynExprData::Prefix {
                                    opr: Minus,
                                    opr_regional_token_idx: RegionalTokenIdx(
                                        16,
                                    ),
                                    opd: 6,
                                },
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        19,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            0,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        21,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            99,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        23,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            2,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        25,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            83,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        27,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            782,
                                        ),
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        29,
                                    ),
                                    LiteralTokenData::Integer(
                                        UnspecifiedRegular(
                                            1,
                                        ),
                                    ),
                                ),
                                SynExprData::List {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 5,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 7,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 8,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    20,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 9,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    22,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 10,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    24,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 11,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    26,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 12,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    28,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 13,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        30,
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
                            ],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
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
                                                2,
                                            ),
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    initial_value: 14,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `v`,
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
                                        `v`,
                                        0,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
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
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    31,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `v`,
                                            pattern_variable_idx: 0,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [
                                (
                                    LetPattern {
                                        pattern: LetPatternSynExprRoot {
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
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 0,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtType,
                                syn_expr_idx: 2,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 14,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 15,
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
    (
        ItemSynNodePath::Attr(
            Room32,
            AttrSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Attr(
                        AttrSynNodePathData {
                            parent_syn_node_path: ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                            attr_item_path_result: Ok(
                                AttrItemPath(`quick_sort::quick_sort_works_for_integers::#test(0)`),
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Form(
                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                    Fn,
                )`, (0)),
            ),
        ),
        Some(
            ItemSynNodeDefn {
                body: 7,
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: Some(
                            SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::ItemDecl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Form(
                                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                                    Fn,
                                                )`, (0)),
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
                                    pattern_expr_region: SynPatternRegion {
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
                                        inherited_variable_arena: Arena {
                                            data: [],
                                        },
                                        current_variable_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: False,
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
                        ),
                        path: SynNodeRegionPath::ItemDefn(
                            ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "beach",
                                        },
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "hotel",
                                        },
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "airplane",
                                        },
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        12,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "car",
                                        },
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        14,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "house",
                                        },
                                    ),
                                ),
                                SynExprData::Literal(
                                    RegionalTokenIdx(
                                        16,
                                    ),
                                    LiteralTokenData::String(
                                        StringLiteralTokenData {
                                            data: "art",
                                        },
                                    ),
                                ),
                                SynExprData::List {
                                    lbox_regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                    items: [
                                        SynCommaListItem {
                                            syn_expr_idx: 0,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    7,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 1,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 2,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 3,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 4,
                                            comma_regional_token_idx: Some(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                        },
                                        SynCommaListItem {
                                            syn_expr_idx: 5,
                                            comma_regional_token_idx: None,
                                        },
                                    ],
                                    rbox_regional_token_idx: RegionalTokenIdx(
                                        17,
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
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [
                                SynStmtData::Let {
                                    let_token: LetRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            1,
                                        ),
                                    },
                                    let_variables_pattern: Ok(
                                        LetPatternSyndicate {
                                            syn_pattern_expr_root: LetPatternSynExprRoot {
                                                syn_pattern_idx: 0,
                                            },
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                            colon_token: Ok(
                                                None,
                                            ),
                                            ty: None,
                                        },
                                    ),
                                    assign_token: Ok(
                                        EqRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                    ),
                                    initial_value: 6,
                                },
                            ],
                        },
                        pattern_expr_region: SynPatternRegion {
                            pattern_expr_arena: Arena {
                                data: [
                                    SynPatternData::Ident {
                                        symbol_modifier_tokens: Some(
                                            Mut(
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        2,
                                                    ),
                                                },
                                            ),
                                        ),
                                        ident_token: IdentRegionalToken {
                                            ident: `strs`,
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
                                        `strs`,
                                        0,
                                    ),
                                ],
                            ],
                            pattern_symbol_modifiers: ArenaMap {
                                data: [
                                    Mut,
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
                                        modifier: Mut,
                                        access_start: RegionalTokenIdx(
                                            4,
                                        ),
                                        access_end: Some(
                                            RegionalTokenIdxRangeEnd(
                                                RegionalTokenIdx(
                                                    18,
                                                ),
                                            ),
                                        ),
                                        data: CurrentVariableData::LetVariable {
                                            ident: `strs`,
                                            pattern_variable_idx: 0,
                                        },
                                    },
                                ],
                            },
                            allow_self_type: False,
                            allow_self_value: False,
                            pattern_ty_constraints: [],
                        },
                        pattern_roots: [
                            SynPatternRoot {
                                kind: SynPatternRootKind::Let,
                                syn_pattern_idx: 0,
                            },
                        ],
                        expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::LetStmtInitialValue,
                                syn_expr_idx: 6,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::RootBody,
                                syn_expr_idx: 7,
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
    (
        ItemSynNodePath::Attr(
            Room32,
            AttrSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Attr(
                        AttrSynNodePathData {
                            parent_syn_node_path: ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Form(
                                    MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                        Fn,
                                    )`, (0)),
                                ),
                            ),
                            attr_item_path_result: Ok(
                                AttrItemPath(`quick_sort::quick_sort_works_for_strs::#test(0)`),
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
]
```