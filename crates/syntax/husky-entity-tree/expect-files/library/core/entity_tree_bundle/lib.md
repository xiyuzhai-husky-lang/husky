Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `core`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `basic`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `default`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `logic`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `mem`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `num`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `ops`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `prelude`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 7,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `raw_bits`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 8,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `fmt`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `clone`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `marker`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 11,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `list`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `cmp`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `str`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `option`,
                            visibility: Visibility::Public,
                            symbol: Submodule(
                                SubmoduleSymbol(
                                    Id {
                                        value: 15,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::basic`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `bool`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `never`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `unit`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 3,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Trait`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Module`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::default`,
                symbols: EntitySymbolTable(
                    [],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::logic`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Prop`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LogicAnd`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 7,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `LogicOr`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 8,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::mem`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Ref`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `RefMut`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Leash`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 11,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::num`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `i8`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i16`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 13,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i32`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i64`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 15,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i128`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 16,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `isize`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u8`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u16`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 19,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u32`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u64`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 21,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u128`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 22,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `usize`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 23,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f32`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 24,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f64`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 25,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Add`,
                            visibility: Visibility::PublicUnder(
                                `core::num`,
                            ),
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 32,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    30,
                                ),
                            },
                            ty_expr: 0,
                            body: ArenaIdxRange(
                                0..1,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 33,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                            trai_expr: 1,
                            for_token: TokenIdx(
                                44,
                            ),
                            ty_expr: 2,
                            body: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 37,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    82,
                                ),
                            },
                            ty_expr: 3,
                            body: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 38,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                            trai_expr: 4,
                            for_token: TokenIdx(
                                96,
                            ),
                            ty_expr: 5,
                            body: ArenaIdxRange(
                                3..4,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 42,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    134,
                                ),
                            },
                            ty_expr: 6,
                            body: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 43,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    145,
                                ),
                            },
                            trai_expr: 7,
                            for_token: TokenIdx(
                                148,
                            ),
                            ty_expr: 8,
                            body: ArenaIdxRange(
                                5..6,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 47,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    185,
                                ),
                            },
                            ty_expr: 9,
                            body: ArenaIdxRange(
                                6..7,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 48,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    196,
                                ),
                            },
                            trai_expr: 10,
                            for_token: TokenIdx(
                                199,
                            ),
                            ty_expr: 11,
                            body: ArenaIdxRange(
                                7..8,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 52,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    237,
                                ),
                            },
                            ty_expr: 12,
                            body: ArenaIdxRange(
                                8..9,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 53,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    248,
                                ),
                            },
                            trai_expr: 13,
                            for_token: TokenIdx(
                                251,
                            ),
                            ty_expr: 14,
                            body: ArenaIdxRange(
                                9..10,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 57,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    289,
                                ),
                            },
                            ty_expr: 15,
                            body: ArenaIdxRange(
                                10..11,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 58,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    300,
                                ),
                            },
                            trai_expr: 16,
                            for_token: TokenIdx(
                                303,
                            ),
                            ty_expr: 17,
                            body: ArenaIdxRange(
                                11..12,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::u8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 62,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    341,
                                ),
                            },
                            ty_expr: 18,
                            body: ArenaIdxRange(
                                12..13,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::u8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 63,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    352,
                                ),
                            },
                            trai_expr: 19,
                            for_token: TokenIdx(
                                355,
                            ),
                            ty_expr: 20,
                            body: ArenaIdxRange(
                                13..14,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::u16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 67,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    393,
                                ),
                            },
                            ty_expr: 21,
                            body: ArenaIdxRange(
                                14..15,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::u16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 68,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    404,
                                ),
                            },
                            trai_expr: 22,
                            for_token: TokenIdx(
                                407,
                            ),
                            ty_expr: 23,
                            body: ArenaIdxRange(
                                15..16,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::u32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 72,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    445,
                                ),
                            },
                            ty_expr: 24,
                            body: ArenaIdxRange(
                                16..17,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::u32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 73,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    456,
                                ),
                            },
                            trai_expr: 25,
                            for_token: TokenIdx(
                                459,
                            ),
                            ty_expr: 26,
                            body: ArenaIdxRange(
                                17..18,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::u64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 77,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    497,
                                ),
                            },
                            ty_expr: 27,
                            body: ArenaIdxRange(
                                18..19,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::u64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 78,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    508,
                                ),
                            },
                            trai_expr: 28,
                            for_token: TokenIdx(
                                511,
                            ),
                            ty_expr: 29,
                            body: ArenaIdxRange(
                                19..20,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::u128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 82,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    549,
                                ),
                            },
                            ty_expr: 30,
                            body: ArenaIdxRange(
                                20..21,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::u128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 83,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    560,
                                ),
                            },
                            trai_expr: 31,
                            for_token: TokenIdx(
                                563,
                            ),
                            ty_expr: 32,
                            body: ArenaIdxRange(
                                21..22,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 87,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    601,
                                ),
                            },
                            ty_expr: 33,
                            body: ArenaIdxRange(
                                22..23,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::usize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 88,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    612,
                                ),
                            },
                            trai_expr: 34,
                            for_token: TokenIdx(
                                615,
                            ),
                            ty_expr: 35,
                            body: ArenaIdxRange(
                                23..24,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 92,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    653,
                                ),
                            },
                            ty_expr: 36,
                            body: ArenaIdxRange(
                                24..25,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 93,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    664,
                                ),
                            },
                            trai_expr: 37,
                            for_token: TokenIdx(
                                667,
                            ),
                            ty_expr: 38,
                            body: ArenaIdxRange(
                                25..26,
                            ),
                        },
                    ),
                    ImplBlock::Type(
                        TypeImplBlock {
                            id: TypeImplBlockId {
                                module_path: `core::num`,
                                ty_path: TypePath(`core::num::f64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 97,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    704,
                                ),
                            },
                            ty_expr: 39,
                            body: ArenaIdxRange(
                                26..27,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::num`,
                                trai_path: TraitPath(`core::ops::Add`),
                                ty_path: TypePath(`core::num::f64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 98,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    715,
                                ),
                            },
                            trai_expr: 40,
                            for_token: TokenIdx(
                                718,
                            ),
                            ty_expr: 41,
                            body: ArenaIdxRange(
                                27..28,
                            ),
                        },
                    ),
                ],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 28,
                            use_expr_idx: 2,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `core::num`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 28,
                            use_expr_idx: 1,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `core::num`,
                                ),
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `ops`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
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
                        UseExprRule {
                            ast_idx: 28,
                            use_expr_idx: 0,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::PublicUnder(
                                    `core::num`,
                                ),
                            },
                            variant: UseExprRuleVariant::Leaf {
                                ident_token: IdentToken {
                                    ident: `Add`,
                                    token_idx: TokenIdx(
                                        5,
                                    ),
                                },
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core::ops`,
                                ),
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
                    ],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::ops`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Add`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 26,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `AddAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 27,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitAnd`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 28,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitAndAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 29,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitOr`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 30,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitOrAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 31,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitXor`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `BitXorAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 33,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Div`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `DivAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Mul`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `MulAssign`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Neg`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Not`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Sub`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::prelude`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `bool`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 32,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `never`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 33,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `unit`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 34,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Trait`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 35,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Module`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i8`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i16`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 38,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i32`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i64`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `i128`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `isize`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u8`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u16`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u32`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 45,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u64`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `u128`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `usize`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f32`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 49,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `f64`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `r32`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Debug`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Copy`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 54,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Sized`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 55,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Clone`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 56,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialEq`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 57,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Eq`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 58,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialOrd`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 59,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Ord`,
                            visibility: Visibility::Public,
                            symbol: Use(
                                UseSymbol(
                                    Id {
                                        value: 60,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseExprRule {
                            ast_idx: 0,
                            use_expr_idx: 2,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    1..2,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 1,
                            use_expr_idx: 5,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    4..5,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 2,
                            use_expr_idx: 8,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            16,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    7..8,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 3,
                            use_expr_idx: 11,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            23,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    10..11,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 4,
                            use_expr_idx: 14,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            30,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    13..14,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 5,
                            use_expr_idx: 17,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            37,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    16..17,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 6,
                            use_expr_idx: 20,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Crate(
                                    CrateToken {
                                        token_idx: TokenIdx(
                                            44,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    19..20,
                                ),
                            },
                            parent: None,
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
                        UseExprRule {
                            ast_idx: 0,
                            use_expr_idx: 1,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `basic`,
                                        token_idx: TokenIdx(
                                            4,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    0..1,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 1,
                            use_expr_idx: 4,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `num`,
                                        token_idx: TokenIdx(
                                            11,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    3..4,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 2,
                            use_expr_idx: 7,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `raw_bits`,
                                        token_idx: TokenIdx(
                                            18,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    6..7,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 3,
                            use_expr_idx: 10,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `fmt`,
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    9..10,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 9,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 4,
                            use_expr_idx: 13,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `marker`,
                                        token_idx: TokenIdx(
                                            32,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    12..13,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 11,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 5,
                            use_expr_idx: 16,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `clone`,
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    15..16,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseExprRule {
                            ast_idx: 6,
                            use_expr_idx: 19,
                            accessibility: VisibilityProgress::Done {
                                accessibility: Visibility::Public,
                            },
                            variant: UseExprRuleVariant::Parent {
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `cmp`,
                                        token_idx: TokenIdx(
                                            46,
                                        ),
                                    },
                                ),
                                children: ArenaIdxRange(
                                    18..19,
                                ),
                            },
                            parent: Some(
                                EntityPath::Module(
                                    `core`,
                                ),
                            ),
                            state: UseExprRuleState::Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 13,
                                        },
                                    ),
                                ),
                            },
                        },
                    ],
                ),
                use_all_rules: UseAllRules(
                    [
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            },
                            ast_idx: 0,
                            use_expr_idx: 0,
                            visibility: Public,
                            progress: 5,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            },
                            ast_idx: 1,
                            use_expr_idx: 3,
                            visibility: Public,
                            progress: 15,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            },
                            ast_idx: 2,
                            use_expr_idx: 6,
                            visibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            },
                            ast_idx: 3,
                            use_expr_idx: 9,
                            visibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            },
                            ast_idx: 4,
                            use_expr_idx: 12,
                            visibility: Public,
                            progress: 2,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 11,
                                    },
                                ),
                            },
                            ast_idx: 5,
                            use_expr_idx: 15,
                            visibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: KinshipedModulePath {
                                kinship: Inside,
                                path: ModulePath(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            },
                            ast_idx: 6,
                            use_expr_idx: 18,
                            visibility: Public,
                            progress: 4,
                        },
                    ],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::raw_bits`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `r32`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 41,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::fmt`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Debug`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::clone`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Clone`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 43,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::marker`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Copy`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 44,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Sized`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 45,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::list`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `List`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 46,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::cmp`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `PartialEq`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Eq`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 48,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `PartialOrd`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 49,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `Ord`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 50,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::str`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `str`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 51,
                                    },
                                ),
                            ),
                        },
                        EntitySymbolEntry {
                            ident: `StringLiteral`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 52,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
            EntityTreeSheet {
                module_path: `core::option`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `Option`,
                            visibility: Visibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol(
                                    Id {
                                        value: 53,
                                    },
                                ),
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [],
            },
        ],
        principal_entity_path_expr_arena: Arena {
            data: [
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i8`,
                        token_idx: TokenIdx(
                            31,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            42,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i8`,
                        token_idx: TokenIdx(
                            45,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i8`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i16`,
                        token_idx: TokenIdx(
                            83,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            94,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i16`,
                        token_idx: TokenIdx(
                            97,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i16`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i32`,
                        token_idx: TokenIdx(
                            135,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            146,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i32`,
                        token_idx: TokenIdx(
                            149,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i64`,
                        token_idx: TokenIdx(
                            186,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            197,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i64`,
                        token_idx: TokenIdx(
                            200,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i64`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i128`,
                        token_idx: TokenIdx(
                            238,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            249,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i128`,
                        token_idx: TokenIdx(
                            252,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::i128`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `isize`,
                        token_idx: TokenIdx(
                            290,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            301,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `isize`,
                        token_idx: TokenIdx(
                            304,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u8`,
                        token_idx: TokenIdx(
                            342,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            353,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u8`,
                        token_idx: TokenIdx(
                            356,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u8`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u16`,
                        token_idx: TokenIdx(
                            394,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            405,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u16`,
                        token_idx: TokenIdx(
                            408,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u16`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u32`,
                        token_idx: TokenIdx(
                            446,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            457,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u32`,
                        token_idx: TokenIdx(
                            460,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u64`,
                        token_idx: TokenIdx(
                            498,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            509,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u64`,
                        token_idx: TokenIdx(
                            512,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u64`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u128`,
                        token_idx: TokenIdx(
                            550,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            561,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `u128`,
                        token_idx: TokenIdx(
                            564,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::u128`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `usize`,
                        token_idx: TokenIdx(
                            602,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            613,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `usize`,
                        token_idx: TokenIdx(
                            616,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::usize`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `f32`,
                        token_idx: TokenIdx(
                            654,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            665,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `f32`,
                        token_idx: TokenIdx(
                            668,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f32`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `f64`,
                        token_idx: TokenIdx(
                            705,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `Add`,
                        token_idx: TokenIdx(
                            716,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::ops::Add`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `f64`,
                        token_idx: TokenIdx(
                            719,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::f64`, `Extern`),
                        ),
                    ),
                },
            ],
        },
    },
)