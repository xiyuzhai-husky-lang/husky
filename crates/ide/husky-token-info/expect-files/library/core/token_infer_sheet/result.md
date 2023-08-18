Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
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
                use_expr_idx: 3,
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
                                ast_idx: 7,
                                ident_token: IdentToken {
                                    ident: `Result`,
                                    token_idx: TokenIdx(
                                        11,
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
                                                0..2,
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
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T`,
                            token_idx: TokenIdx(
                                13,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                15,
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
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                31,
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
                            ident: `E1`,
                            token_idx: TokenIdx(
                                33,
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
                            ident: `E2`,
                            token_idx: TokenIdx(
                                35,
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
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                31,
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
                            ident: `E2`,
                            token_idx: TokenIdx(
                                35,
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
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E1`,
                            token_idx: TokenIdx(
                                33,
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
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSynSymbol::Type {
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
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 0,
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
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSynSymbol::Type {
                        ident: `T2`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSynSymbol::Type {
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
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSynSymbol::Type {
                        ident: `T1`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSynSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSynSymbol::Type {
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