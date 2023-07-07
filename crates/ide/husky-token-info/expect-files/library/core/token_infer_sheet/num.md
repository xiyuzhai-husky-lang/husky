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
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: OnceUseRuleIdx(
                    1,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: `core::ops`,
                            node: SubmoduleNode {
                                node_path: SubmoduleNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: `core::ops`,
                                        disambiguator: 0,
                                    },
                                },
                                visibility: Scope::Pub,
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `ops`,
                                    token_idx: TokenIdx(
                                        35,
                                    ),
                                },
                            },
                        },
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 0,
                rule_idx: OnceUseRuleIdx(
                    2,
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::ModuleItem {
                            module_item_path: ModuleItemPath::Trait(
                                TraitPath(`core::ops::Add`),
                            ),
                            node: ModuleItemNode {
                                node_path: ModuleItemNodePath::Trait(
                                    TraitNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TraitPath(`core::ops::Add`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 30,
                                ident_token: IdentToken {
                                    ident: `Add`,
                                    token_idx: TokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                    ),
                },
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
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::i8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i8`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::i16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i16`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i32`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::i64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i64`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::i128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::i128`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::isize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::isize`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::u8`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::u8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::u8`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u8`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::u16`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::u16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::u16`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u16`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::u32`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::u32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::u32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u32`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::u64`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::u64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::u64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u64`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::u128`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::u128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::u128`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::u128`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::usize`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::usize`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::usize`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::f32`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `sqrt`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f32`, `Extern`),
                    ),
                ),
            ),
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
            TokenInfo::EntityNode(
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Type(
                        TypeNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::num::f64`, `Extern`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                ModuleItem {
                    module_item_kind: Type(
                        Extern,
                    ),
                    connection: Connected,
                },
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TypeItem(
                        TypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::num`,
                                        ty_path: TypePath(`core::num::f64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `abs`,
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
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Trait(
                        TraitPath(`core::ops::Add`),
                    ),
                ),
            ),
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::EntityNode(
                EntityNodePath::AssociatedItem(
                    AssociatedItemNodePath::TraitForTypeItem(
                        TraitForTypeItemNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::num`,
                                        trai_path: TraitPath(`core::ops::Add`),
                                        ty_path: TypePath(`core::num::f64`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `add`,
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
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`core::num::f64`, `Extern`),
                    ),
                ),
            ),
            TokenInfo::None,
        ],
    },
)