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
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 0,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    41,
                                ),
                            },
                            ast_idx: 33,
                            body: ArenaIdxRange(
                                1..2,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                42,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 1,
                            body: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 1,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    93,
                                ),
                            },
                            ast_idx: 38,
                            body: ArenaIdxRange(
                                3..4,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 2,
                            body: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 2,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    145,
                                ),
                            },
                            ast_idx: 43,
                            body: ArenaIdxRange(
                                5..6,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 3,
                            body: ArenaIdxRange(
                                6..7,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 3,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    196,
                                ),
                            },
                            ast_idx: 48,
                            body: ArenaIdxRange(
                                7..8,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                197,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 4,
                            body: ArenaIdxRange(
                                8..9,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 4,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    248,
                                ),
                            },
                            ast_idx: 53,
                            body: ArenaIdxRange(
                                9..10,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                249,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 5,
                            body: ArenaIdxRange(
                                10..11,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 5,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    300,
                                ),
                            },
                            ast_idx: 58,
                            body: ArenaIdxRange(
                                11..12,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                301,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 6,
                            body: ArenaIdxRange(
                                12..13,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 6,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    352,
                                ),
                            },
                            ast_idx: 63,
                            body: ArenaIdxRange(
                                13..14,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                353,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 7,
                            body: ArenaIdxRange(
                                14..15,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 7,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    404,
                                ),
                            },
                            ast_idx: 68,
                            body: ArenaIdxRange(
                                15..16,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                405,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 8,
                            body: ArenaIdxRange(
                                16..17,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 8,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    456,
                                ),
                            },
                            ast_idx: 73,
                            body: ArenaIdxRange(
                                17..18,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                457,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 9,
                            body: ArenaIdxRange(
                                18..19,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 9,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    508,
                                ),
                            },
                            ast_idx: 78,
                            body: ArenaIdxRange(
                                19..20,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                509,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 10,
                            body: ArenaIdxRange(
                                20..21,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 10,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    560,
                                ),
                            },
                            ast_idx: 83,
                            body: ArenaIdxRange(
                                21..22,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                561,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 11,
                            body: ArenaIdxRange(
                                22..23,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 11,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    612,
                                ),
                            },
                            ast_idx: 88,
                            body: ArenaIdxRange(
                                23..24,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                613,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 12,
                            body: ArenaIdxRange(
                                24..25,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 12,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    664,
                                ),
                            },
                            ast_idx: 93,
                            body: ArenaIdxRange(
                                25..26,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                665,
                                            ),
                                        },
                                    ),
                                ),
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
                            ty_expr: 13,
                            body: ArenaIdxRange(
                                26..27,
                            ),
                        },
                    ),
                    ImplBlock::IllFormed(
                        IllFormedImplBlock {
                            id: IllFormedImplBlockId {
                                module: `core::num`,
                                disambiguator: 13,
                            },
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    715,
                                ),
                            },
                            ast_idx: 98,
                            body: ArenaIdxRange(
                                27..28,
                            ),
                            ill_form: ImplBlockIllForm::MajorPath(
                                MajorPathExprError::Original(
                                    OriginalMajorPathExprError::UnrecognizedIdent(
                                        IdentToken {
                                            ident: `Add`,
                                            token_idx: TokenIdx(
                                                716,
                                            ),
                                        },
                                    ),
                                ),
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
                                parent_name_token: NameToken::Ident(
                                    IdentToken {
                                        ident: `core`,
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
                            state: UseExprRuleState::Erroneous,
                        },
                    ],
                ),
                use_all_rules: UseAllRules(
                    [],
                ),
                errors: [
                    EntityTreeError::Original(
                        OriginalEntityTreeError::UnresolvedIdent(
                            IdentToken {
                                ident: `core`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        ),
                    ),
                ],
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
                                        value: 29,
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
                                        value: 30,
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
                                        value: 31,
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
                                        value: 32,
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
                                        value: 33,
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
                                        value: 34,
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
                                        value: 35,
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
                                        value: 36,
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
                                        value: 37,
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
                                        value: 38,
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
                                        value: 39,
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
                                        value: 40,
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
                                        value: 41,
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
                                        value: 42,
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
                                        value: 43,
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
                                        value: 44,
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
                                        value: 45,
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
                                        value: 46,
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
                                        value: 47,
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
                                        value: 48,
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
                                        value: 49,
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
                                        value: 50,
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
                                        value: 51,
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
                                        value: 52,
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
                                        value: 53,
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
                                        value: 54,
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
                                        value: 55,
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
                                        value: 56,
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
                            parent: ModulePath(
                                Id {
                                    value: 2,
                                },
                            ),
                            ast_idx: 0,
                            use_expr_idx: 0,
                            accessibility: Public,
                            progress: 5,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 6,
                                },
                            ),
                            ast_idx: 1,
                            use_expr_idx: 3,
                            accessibility: Public,
                            progress: 14,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 9,
                                },
                            ),
                            ast_idx: 2,
                            use_expr_idx: 6,
                            accessibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 10,
                                },
                            ),
                            ast_idx: 3,
                            use_expr_idx: 9,
                            accessibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 12,
                                },
                            ),
                            ast_idx: 4,
                            use_expr_idx: 12,
                            accessibility: Public,
                            progress: 2,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 11,
                                },
                            ),
                            ast_idx: 5,
                            use_expr_idx: 15,
                            accessibility: Public,
                            progress: 1,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 14,
                                },
                            ),
                            ast_idx: 6,
                            use_expr_idx: 18,
                            accessibility: Public,
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
                impl_blocks: [
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 8,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                            trai_expr: 14,
                            for_token: TokenIdx(
                                13,
                            ),
                            ty_expr: 15,
                            body: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 9,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                            trai_expr: 16,
                            for_token: TokenIdx(
                                25,
                            ),
                            ty_expr: 17,
                            body: ArenaIdxRange(
                                2..3,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 10,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    35,
                                ),
                            },
                            trai_expr: 18,
                            for_token: TokenIdx(
                                37,
                            ),
                            ty_expr: 19,
                            body: ArenaIdxRange(
                                3..4,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 11,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    47,
                                ),
                            },
                            trai_expr: 20,
                            for_token: TokenIdx(
                                49,
                            ),
                            ty_expr: 21,
                            body: ArenaIdxRange(
                                4..5,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 12,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    59,
                                ),
                            },
                            trai_expr: 22,
                            for_token: TokenIdx(
                                61,
                            ),
                            ty_expr: 23,
                            body: ArenaIdxRange(
                                5..6,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::clone`,
                                trai_path: TraitPath(`core::clone::Clone`),
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 13,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    71,
                                ),
                            },
                            trai_expr: 24,
                            for_token: TokenIdx(
                                73,
                            ),
                            ty_expr: 25,
                            body: ArenaIdxRange(
                                6..7,
                            ),
                        },
                    ),
                ],
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
                impl_blocks: [
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::i8`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 1,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    4,
                                ),
                            },
                            trai_expr: 26,
                            for_token: TokenIdx(
                                6,
                            ),
                            ty_expr: 27,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::i16`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 2,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    8,
                                ),
                            },
                            trai_expr: 28,
                            for_token: TokenIdx(
                                10,
                            ),
                            ty_expr: 29,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 3,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    12,
                                ),
                            },
                            trai_expr: 30,
                            for_token: TokenIdx(
                                14,
                            ),
                            ty_expr: 31,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::i64`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 4,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    16,
                                ),
                            },
                            trai_expr: 32,
                            for_token: TokenIdx(
                                18,
                            ),
                            ty_expr: 33,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::i128`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 5,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    20,
                                ),
                            },
                            trai_expr: 34,
                            for_token: TokenIdx(
                                22,
                            ),
                            ty_expr: 35,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                    ImplBlock::TraitForType(
                        TraitForTypeImplBlock {
                            id: TraitForTypeImplBlockId {
                                module_path: `core::marker`,
                                trai_path: TraitPath(`core::marker::Copy`),
                                ty_path: TypePath(`core::num::isize`, `Extern`),
                                disambiguator: 0,
                            },
                            ast_idx: 6,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    24,
                                ),
                            },
                            trai_expr: 36,
                            for_token: TokenIdx(
                                26,
                            ),
                            ty_expr: 37,
                            body: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ),
                ],
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            12,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i8`,
                        token_idx: TokenIdx(
                            14,
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            24,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i16`,
                        token_idx: TokenIdx(
                            26,
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            36,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i32`,
                        token_idx: TokenIdx(
                            38,
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            48,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i64`,
                        token_idx: TokenIdx(
                            50,
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            60,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i128`,
                        token_idx: TokenIdx(
                            62,
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
                        ident: `Clone`,
                        token_idx: TokenIdx(
                            72,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::clone::Clone`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `isize`,
                        token_idx: TokenIdx(
                            74,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            5,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i8`,
                        token_idx: TokenIdx(
                            7,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            9,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i16`,
                        token_idx: TokenIdx(
                            11,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            13,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i32`,
                        token_idx: TokenIdx(
                            15,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            17,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i64`,
                        token_idx: TokenIdx(
                            19,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            21,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `i128`,
                        token_idx: TokenIdx(
                            23,
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
                        ident: `Copy`,
                        token_idx: TokenIdx(
                            25,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Trait(
                            TraitPath(`core::marker::Copy`),
                        ),
                    ),
                },
                MajorPathExpr::Root {
                    ident_token: IdentToken {
                        ident: `isize`,
                        token_idx: TokenIdx(
                            27,
                        ),
                    },
                    entity_path: EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`core::num::isize`, `Extern`),
                        ),
                    ),
                },
            ],
        },
    },
)