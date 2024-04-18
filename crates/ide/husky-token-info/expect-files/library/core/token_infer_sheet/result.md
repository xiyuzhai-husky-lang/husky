```rust
Ok(
    TokenInfoSheet {
        token_infos: [
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        1,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 1,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::CrateRoot {
                                    root_module_path: `core`,
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        3,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 3,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`core::result::Result`, `Enum`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExprStar,
                },
            ),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 0,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        2,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::TemplateParameter(
                        3,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 3,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::Module(
                            `core`,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            `core`,
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            `core::ops`,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            `core::ops`,
                        ),
                    ),
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
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
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 1,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 3,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 0,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 2,
                        current_syn_symbol_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
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
                                                            `<core::result::Result as core::ops::Unveil(0)>::Continue`,
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
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 3,
                        inherited_syn_symbol_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::AstIdentifiable,
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
                                                            `<core::result::Result as core::ops::Unveil(0)>::unveil`,
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
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::PatternExpr(
                        0,
                    ),
                    data: TokenInfoData::CurrentSynSymbol {
                        current_syn_symbol_idx: 0,
                        current_syn_symbol_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_symbol_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 1,
                        inherited_syn_symbol_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `T2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 3,
                        inherited_syn_symbol_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
            None,
            Some(
                TokenInfo {
                    src: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 0,
                        inherited_syn_symbol_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `T1`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            Some(
                TokenInfo {
                    src: TokenInfoSource::SemaExpr(
                        SemaExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedSynSymbol {
                        inherited_syn_symbol_idx: 2,
                        inherited_syn_symbol_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E1`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ),
            None,
        ],
    },
)
```