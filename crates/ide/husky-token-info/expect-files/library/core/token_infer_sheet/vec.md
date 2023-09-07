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
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::fmt::Debug`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::clone::Clone`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::visual::Visualize`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::vec::Vec`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                MajorItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                19,
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
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                24,
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
                        TypePath(`core::vec::Vec`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                        ident_token: IdentToken {
                            ident: `E`,
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    },
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `ilen`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `push`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 0,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `first`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `last`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `pop`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::option::Option`, `Enum`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `collect_leashes`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::BoxPrefix,
            TokenInfo::BoxPrefix,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `cyclic_slice_leashed`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
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
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
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
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                ItemSynNodePath::AssociatedItem(
                    AssociatedItemSynNodePath::TypeItem(
                        TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::vec`,
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `pop_with_largest_opt_f32`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                AssociatedItem {
                    associated_item_kind: TypeItem(
                        MethodFn,
                    ),
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {
                    pattern_symbol_idx: 0,
                },
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::InheritedSymbol {
                inherited_symbol_idx: 0,
                inherited_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                    InheritedTemplateParameterSynSymbol::Type {
                        ident: `E`,
                    },
                ),
                syn_expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
        ],
    },
)