```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class::#derive(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
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
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class::#derive(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class::#derive(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::Class`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Type(
                                TypeKind::Enum,
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
                                    ident: `Label`,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class::Known`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `Label`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll::#derive(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
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
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll::#derive(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll::#derive(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Type(
                                TypeKind::Enum,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
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
                            7,
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
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                ident_token: IdentRegionalToken {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
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
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
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
            [],
            [],
            [],
            [],
            [],
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
                                    ident: `Label`,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                            ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::default::Default`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::default::Default`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::default::Default`),
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
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
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
                            ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        },
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
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::SelfType,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            1,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::No`),
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
                            3,
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
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
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
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
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
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::Class as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssocItem(
                            AssocItemSynNodePath::TraitForTypeItem(
                                TraitForTypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssocItem(
                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                TraitForTypeItemSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                            `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                                            TraitItemKind::AssocType,
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
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocType,
                            ),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
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
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
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
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `Label`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Constant {
                                ident: `label`,
                            },
                        ),
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
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
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
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::SelfType,
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `one_vs_all`,
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
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
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
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
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
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
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
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::Class::Known`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::Class::Known`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::Class::Known`),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Constant {
                                ident: `label`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            23,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        9,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
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
                            35,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            35,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        13,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
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
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            37,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            38,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            39,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAllResult::#derive(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
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
                            6,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAllResult::#derive(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
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
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAllResult::#derive(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Type(
                                TypeKind::Enum,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
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
                            7,
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
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Constant {
                                ident_token: IdentRegionalToken {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
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
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAllResult`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
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
            [],
            [],
            [],
            [],
            [],
            [],
            [],
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
                                    ident: `Label`,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            23,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            11,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssocItem(
                            AssocItemSynNodePath::TraitForTypeItem(
                                TraitForTypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssocItem(
                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                TraitForTypeItemSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                            `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                                            TraitItemKind::AssocType,
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
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocType,
                            ),
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
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
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
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
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
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
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `Label`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Constant {
                                ident: `LABEL`,
                            },
                        ),
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
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
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
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::SelfType,
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
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
                        RegionPath::ItemDecl(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
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
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Parenate {
                            ident: `one_vs_all_result`,
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
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
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
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        5,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
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
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        6,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            2,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            19,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        7,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::Yes`),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        8,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                            24,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        9,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        10,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            28,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        11,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        12,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
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
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        13,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Break`),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            33,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            34,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        14,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            36,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        15,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAll::No`),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            37,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            39,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        16,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                            41,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        17,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        18,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        19,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
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
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        20,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::ops::ControlFlow`, `Enum`),
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
                            49,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            49,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        21,
                        PrincipalEntityPath::TypeVariant(
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::TypeVariant(
                            Room32,
                            TypeVariantPath(`core::ops::ControlFlow::Continue`),
                        ),
                    ),
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
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
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
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitLeftParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            52,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::UnitRightParenthesis,
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            53,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDefn(
                            ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            9,
                        ),
                    ),
                    data: TokenInfoData::CallPar,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down::#dep(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`core::task::Task`, `TypeVar`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`core::task::Task`, `TypeVar`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`core::task::Task`, `TypeVar`),
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
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Form(
                                MajorFormSynNodePath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`, (0)),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Form(
                                MajorFormKind::Ritchie(
                                    RitchieItemKind::Gn,
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
                                    ident: `Label`,
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
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            10,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
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
                            18,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::VariadicParenateParameter {
                            ident_token: IdentRegionalToken {
                                ident: `f`,
                                regional_token_idx: RegionalTokenIdx(
                                    18,
                                ),
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
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
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
                        0,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
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
                            23,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
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
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            2,
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
                            25,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
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
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            27,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::Literal,
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            31,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            4,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            31,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            32,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            33,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`malamute::narrow_down`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
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
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
        ],
    },
)
```