Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 4,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            node: MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::result::Result`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 8,
                                ident_token: IdentToken {
                                    ident: `Result`,
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                block: Type {
                                    path: TypePath(
                                        Id {
                                            value: 30,
                                        },
                                    ),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExprStar,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T`,
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                32,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E1`,
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E2`,
                            token_idx: TokenIdx(
                                36,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::Module(
                    `core`,
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::Module(
                    `core::ops`,
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                32,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 4,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E2`,
                            token_idx: TokenIdx(
                                36,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E1`,
                            token_idx: TokenIdx(
                                34,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TraitForTypeItem(
                        TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Continue`,
                                    item_kind: AssociatedType,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        AssociatedType,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 4,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E2`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TraitForTypeItem(
                        TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `branch`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TraitForTypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 1,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `T2`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 4,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E2`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `T1`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E1`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Todo,
        ],
    },
)