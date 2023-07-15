Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    0,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            node: ModuleItemNode {
                                node_path: ModuleItemNodePath::Type(
                                    TypeNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::result::Result`, `Enum`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 6,
                                ident_token: IdentToken {
                                    ident: `Result`,
                                    token_idx: TokenIdx(
                                        7,
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
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Enum,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T`,
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                11,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
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
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                27,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E1`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E2`,
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 1,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T2`,
                            token_idx: TokenIdx(
                                27,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 3,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E2`,
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `T1`,
                            token_idx: TokenIdx(
                                25,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 2,
                current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                    implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E1`,
                            token_idx: TokenIdx(
                                29,
                            ),
                        },
                    },
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_path: TypePath(`core::result::Result`, `Enum`),
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
                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSymbol::Type {
                        ident: `E2`,
                    },
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_path: TypePath(`core::result::Result`, `Enum`),
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
                current_symbol_kind: CurrentSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 1,
                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSymbol::Type {
                        ident: `T2`,
                    },
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 3,
                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSymbol::Type {
                        ident: `E2`,
                    },
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSymbol::Type {
                        ident: `T1`,
                    },
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 2,
                inherited_symbol_kind: InheritedSymbolKind::ImplicitParameter(
                    InheritedImplicitParameterSymbol::Type {
                        ident: `E1`,
                    },
                ),
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
        ],
    },
)