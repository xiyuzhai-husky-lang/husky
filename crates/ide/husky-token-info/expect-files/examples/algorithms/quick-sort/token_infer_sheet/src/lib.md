```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`quick_sort::quick_sort`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `low`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `high`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::partition`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::partition`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::partition`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `low`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `high`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `low`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            23,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            26,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            14,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            29,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            31,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            33,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            35,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            20,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `high`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_aux`),
                        ),
                        SemExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`quick_sort::partition`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::BoxColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            2,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `high`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `low`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `high`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            26,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            30,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            30,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            31,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            15,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            33,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            16,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            34,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            35,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            17,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            18,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            38,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            20,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            40,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            21,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            41,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            23,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            43,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            24,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            45,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            26,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            47,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            27,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            49,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            29,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            50,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            33,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            51,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            31,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            53,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            30,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            53,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            54,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            33,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            56,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            34,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            57,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            36,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            58,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            35,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            59,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            36,
                        ),
                    ),
                    data: TokenInfoData::IndexColon,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            61,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            39,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            63,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            40,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            65,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            42,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            67,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            43,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            72,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            45,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            74,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            52,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            76,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            47,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            78,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            46,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            78,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            80,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            50,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 2,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            82,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            49,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            82,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            84,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            53,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `arr`,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            86,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            60,
                        ),
                    ),
                    data: TokenInfoData::Method,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            88,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            55,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            90,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            54,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            90,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            92,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            58,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            94,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            57,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            94,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            96,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::partition`),
                        ),
                        SemExprIdx(
                            61,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 1,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::VecFunctorBoxPrefix,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                    ),
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            23,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            12,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            29,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_integers`),
                        ),
                        SemExprIdx(
                            13,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                    Fn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::LetVariable {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`quick_sort::quick_sort_works_for_strs`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
        ],
    },
)
```