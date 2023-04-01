Ok(
    TokenInfoSheet {
        token_infos: [
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 2,
                rule_idx: UseExprRuleIdx(
                    0,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: CrateRoot {
                        root_module: ModulePath(
                            Id {
                                value: 1,
                            },
                        ),
                    },
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 1,
                rule_idx: UseExprRuleIdx(
                    1,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: Submodule(
                        SubmoduleSymbol(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                },
            },
            TokenInfo::None,
            TokenInfo::UseExpr {
                use_expr_idx: 0,
                rule_idx: UseExprRuleIdx(
                    2,
                ),
                state: UseExprRuleState::Resolved {
                    original_symbol: ModuleItem(
                        ModuleItemSymbol(
                            Id {
                                value: 26,
                            },
                        ),
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::i8`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::i8`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::i16`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::i16`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::i32`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::i32`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::i64`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::i64`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::i128`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::i128`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::isize`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::isize`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::u8`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::u8`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::u16`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::u16`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::u32`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::u32`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::u64`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::u64`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::u128`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::u128`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::usize`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::usize`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::f32`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::f32`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                ),
                None,
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
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                Some(
                    ModuleItem {
                        module_item_kind: Type(
                            Extern,
                        ),
                        connection: Connected,
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TypeItem(
                            TypeItemPath {
                                parent_ty: TypePath(`core::num::f64`, `Extern`),
                                ident: `abs`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::AssociatedItem(
                        AssociatedItemPath::TraitForTypeItem(
                            TraitForTypeItemPath {
                                parent_ty: TypePath(`core::num::f64`, `Extern`),
                                trai: TraitPath(`core::ops::Add`),
                                ident: `add`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                ),
                Some(
                    AssociatedItem {
                        associated_item_kind: TraitForTypeItem(
                            MethodFn,
                        ),
                    },
                ),
            ),
            TokenInfo::None,
            TokenInfo::CurrentSymbol {
                current_symbol_idx: 0,
                current_symbol_kind: CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: 0,
                },
                expr_region: ExprRegionLeash(_),
            },
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
            TokenInfo::None,
            TokenInfo::Entity(
                Some(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                ),
                None,
            ),
            TokenInfo::None,
        ],
    },
)