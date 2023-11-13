EntitySynTreeSheet {
    module_path: `core::prelude`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `bool`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::bool`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::bool`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `never`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::never`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::never`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `unit`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::unit`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::unit`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Trait`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::Trait`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Trait`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Module`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::Module`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Module`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Lifetime`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::Lifetime`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Lifetime`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Place`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::basic::Place`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::basic::Place`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 1,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i8`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i16`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i32`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i64`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `i128`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::i128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `isize`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::isize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u8`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u8`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u16`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u16`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u32`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u64`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `u128`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::u128`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `usize`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::usize`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f32`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `f64`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::num::f64`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        use_expr_idx: 4,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `r32`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::raw_bits::r32`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 3,
                        use_expr_idx: 7,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Debug`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::fmt::Debug`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        use_expr_idx: 10,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Visualize`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 5,
                        use_expr_idx: 13,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Html`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::visual::Html`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::visual::Html`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 5,
                        use_expr_idx: 13,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Copy`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Copy`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 6,
                        use_expr_idx: 16,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Sized`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::marker::Sized`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::marker::Sized`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 6,
                        use_expr_idx: 16,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Clone`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::clone::Clone`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 7,
                        use_expr_idx: 19,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Option`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::option::Option`, `Enum`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 8,
                        use_expr_idx: 22,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Some`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::TypeVariant {
                                    ty_variant_path: TypeVariantPath {
                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                        ident: `Some`,
                                    },
                                },
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                        ident: `Some`,
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 4,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                ident: `Some`,
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 8,
                        use_expr_idx: 22,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `None`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::TypeVariant {
                                    ty_variant_path: TypeVariantPath {
                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                        ident: `None`,
                                    },
                                },
                                path: PrincipalEntityPath::TypeVariant(
                                    TypeVariantPath {
                                        parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                        ident: `None`,
                                    },
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 4,
                                use_expr_idx: 3,
                            },
                        ),
                        path: PrincipalEntityPath::TypeVariant(
                            TypeVariantPath {
                                parent_ty_path: TypePath(`core::option::Option`, `Enum`),
                                ident: `None`,
                            },
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 8,
                        use_expr_idx: 22,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialEq`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialEq`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 9,
                        use_expr_idx: 25,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Eq`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Eq`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 9,
                        use_expr_idx: 25,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `PartialOrd`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::PartialOrd`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 9,
                        use_expr_idx: 25,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Ord`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::cmp::Ord`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 9,
                        use_expr_idx: 25,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Slice`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::slice::Slice`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::Slice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 10,
                        use_expr_idx: 28,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `CyclicSlice`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 10,
                        use_expr_idx: 28,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vec`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            module_item_path: MajorItemPath::Type(
                                TypePath(`core::vec::Vec`, `Extern`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::vec::Vec`, `Extern`),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 11,
                        use_expr_idx: 31,
                    },
                ),
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 3,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        2..3,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 2,
                use_expr_idx: 6,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        5..6,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 3,
                use_expr_idx: 9,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                17,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        8..9,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 4,
                use_expr_idx: 12,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        11..12,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 5,
                use_expr_idx: 15,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                31,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        14..15,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 6,
                use_expr_idx: 18,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                38,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        17..18,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 7,
                use_expr_idx: 21,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                45,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        20..21,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 8,
                use_expr_idx: 24,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                52,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        23..24,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 9,
                use_expr_idx: 27,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                59,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        26..27,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 10,
                use_expr_idx: 30,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                66,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        29..30,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 11,
                use_expr_idx: 33,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                73,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        32..33,
                    ),
                },
                parent: None,
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `core`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 2,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `basic`,
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        1..2,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::basic`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 2,
                use_expr_idx: 5,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `num`,
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        4..5,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::num`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 3,
                use_expr_idx: 8,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `raw_bits`,
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        7..8,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::raw_bits`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 4,
                use_expr_idx: 11,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `fmt`,
                            token_idx: TokenIdx(
                                26,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        10..11,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::fmt`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 5,
                use_expr_idx: 14,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `visual`,
                            token_idx: TokenIdx(
                                33,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        13..14,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::visual`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 6,
                use_expr_idx: 17,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `marker`,
                            token_idx: TokenIdx(
                                40,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        16..17,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::marker`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 7,
                use_expr_idx: 20,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `clone`,
                            token_idx: TokenIdx(
                                47,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        19..20,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::clone`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 8,
                use_expr_idx: 23,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `option`,
                            token_idx: TokenIdx(
                                54,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        22..23,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::option`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 9,
                use_expr_idx: 26,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `cmp`,
                            token_idx: TokenIdx(
                                61,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        25..26,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::cmp`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 10,
                use_expr_idx: 29,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `slice`,
                            token_idx: TokenIdx(
                                68,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        28..29,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::slice`,
                            ),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 11,
                use_expr_idx: 32,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `vec`,
                            token_idx: TokenIdx(
                                75,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        31..32,
                    ),
                },
                parent: Some(
                    MajorEntityPath::Module(
                        `core`,
                    ),
                ),
                state: OnceUseRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_path: SubmodulePath(
                                `core::vec`,
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllModuleSymbolsRules(
        [
            UseAllModuleSymbolsRule {
                parent_module_path: `core::basic`,
                is_same_crate: true,
                ast_idx: 1,
                use_expr_idx: 1,
                visibility: Scope::Pub,
                progress: Ok(
                    64,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::num`,
                is_same_crate: true,
                ast_idx: 2,
                use_expr_idx: 4,
                visibility: Scope::Pub,
                progress: Ok(
                    72,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::raw_bits`,
                is_same_crate: true,
                ast_idx: 3,
                use_expr_idx: 7,
                visibility: Scope::Pub,
                progress: Ok(
                    58,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::fmt`,
                is_same_crate: true,
                ast_idx: 4,
                use_expr_idx: 10,
                visibility: Scope::Pub,
                progress: Ok(
                    58,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::visual`,
                is_same_crate: true,
                ast_idx: 5,
                use_expr_idx: 13,
                visibility: Scope::Pub,
                progress: Ok(
                    59,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::marker`,
                is_same_crate: true,
                ast_idx: 6,
                use_expr_idx: 16,
                visibility: Scope::Pub,
                progress: Ok(
                    59,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::clone`,
                is_same_crate: true,
                ast_idx: 7,
                use_expr_idx: 19,
                visibility: Scope::Pub,
                progress: Ok(
                    58,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::option`,
                is_same_crate: true,
                ast_idx: 8,
                use_expr_idx: 22,
                visibility: Scope::Pub,
                progress: Ok(
                    60,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::cmp`,
                is_same_crate: true,
                ast_idx: 9,
                use_expr_idx: 25,
                visibility: Scope::Pub,
                progress: Ok(
                    61,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::slice`,
                is_same_crate: true,
                ast_idx: 10,
                use_expr_idx: 28,
                visibility: Scope::Pub,
                progress: Ok(
                    59,
                ),
            },
            UseAllModuleSymbolsRule {
                parent_module_path: `core::vec`,
                is_same_crate: true,
                ast_idx: 11,
                use_expr_idx: 31,
                visibility: Scope::Pub,
                progress: Ok(
                    58,
                ),
            },
        ],
    ),
    errors: [],
}