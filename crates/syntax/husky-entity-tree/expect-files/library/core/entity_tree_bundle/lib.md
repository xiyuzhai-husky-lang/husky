Ok(
    EntityTreeCrateBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `core`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "basic",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 0,
                                    path: `core::basic`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "logic",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 1,
                                    path: `core::logic`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "mem",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 2,
                                    path: `core::mem`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "num",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 3,
                                    path: `core::num`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "ops",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 4,
                                    path: `core::ops`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 4,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "prelude",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 5,
                                    path: `core::prelude`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 5,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "raw_bits",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 6,
                                    path: `core::raw_bits`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "fmt",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 7,
                                    path: `core::fmt`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 7,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "clone",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 8,
                                    path: `core::clone`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 8,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "marker",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 9,
                                    path: `core::marker`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "vec",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 10,
                                    path: `core::vec`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 10,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "cmp",
                            ),
                            accessibility: Accessibility::PublicUnder(
                                `core`,
                            ),
                            symbol: Submodule(
                                SubmoduleSymbol {
                                    [salsa id]: 11,
                                    path: `core::cmp`,
                                    accessibility: Accessibility::PublicUnder(
                                        `core`,
                                    ),
                                    ast_idx: 11,
                                },
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
                            ident: Identifier(
                                "bool",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 0,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::basic::bool`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "never",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 1,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::basic::never`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "unit",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 2,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::basic::unit`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Trait",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 3,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::basic::Trait`, `Structure`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Module",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 4,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::basic::Module`, `Structure`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 4,
                                },
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
                module_path: `core::logic`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Prop",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 5,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::logic::Prop`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "LogicAnd",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 6,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::logic::LogicAnd`, `Structure`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "LogicOr",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 7,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::logic::LogicOr`, `Inductive`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 2,
                                },
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
                            ident: Identifier(
                                "Ref",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 8,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::mem::Ref`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "RefMut",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 9,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::mem::RefMut`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
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
                            ident: Identifier(
                                "i8",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 10,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::i8`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i16",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 11,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::i16`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 23,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 12,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::i32`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 28,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i64",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 13,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::i64`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 33,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f8",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 14,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::f8`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 38,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f16",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 15,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::f16`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 43,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 16,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::f32`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 48,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f64",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 17,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::num::f64`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 53,
                                },
                            ),
                        },
                    ],
                ),
                impl_blocks: [
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::i8`, `Alien`),
                            },
                        },
                        ast_idx: 19,
                        body: ArenaIdxRange(
                            0..1,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::i8`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 20,
                        body: ArenaIdxRange(
                            1..2,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            36,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::i16`, `Alien`),
                            },
                        },
                        ast_idx: 24,
                        body: ArenaIdxRange(
                            2..3,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::i16`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 25,
                        body: ArenaIdxRange(
                            3..4,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            94,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::i32`, `Alien`),
                            },
                        },
                        ast_idx: 29,
                        body: ArenaIdxRange(
                            4..5,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::i32`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 30,
                        body: ArenaIdxRange(
                            5..6,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            152,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::i64`, `Alien`),
                            },
                        },
                        ast_idx: 34,
                        body: ArenaIdxRange(
                            6..7,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::i64`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 35,
                        body: ArenaIdxRange(
                            7..8,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            209,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::f8`, `Alien`),
                            },
                        },
                        ast_idx: 39,
                        body: ArenaIdxRange(
                            8..9,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::f8`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 40,
                        body: ArenaIdxRange(
                            9..10,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            267,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::f16`, `Alien`),
                            },
                        },
                        ast_idx: 44,
                        body: ArenaIdxRange(
                            10..11,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::f16`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 45,
                        body: ArenaIdxRange(
                            11..12,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            325,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::f32`, `Alien`),
                            },
                        },
                        ast_idx: 49,
                        body: ArenaIdxRange(
                            12..13,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::f32`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 50,
                        body: ArenaIdxRange(
                            13..14,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            383,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Type {
                                ty: TypePath(`core::num::f64`, `Alien`),
                            },
                        },
                        ast_idx: 54,
                        body: ArenaIdxRange(
                            14..15,
                        ),
                        variant: ImplBlockVariant::Type {
                            ty: TypePath(`core::num::f64`, `Alien`),
                        },
                    },
                    ImplBlock {
                        id: ImplBlockId {
                            module_path: `core::num`,
                            impl_block_kind: ImplBlockKind::Err,
                        },
                        ast_idx: 55,
                        body: ArenaIdxRange(
                            15..16,
                        ),
                        variant: ImplBlockVariant::Err(
                            ImplBlockError::MajorPath(
                                UnrecognizedIdentifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 40,
                                                },
                                            ),
                                        ),
                                        token_idx: TokenIdx(
                                            440,
                                        ),
                                    },
                                ),
                            ),
                        ),
                    },
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
                module_path: `core::ops`,
                symbols: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Add",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 18,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::ops::Add`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 5,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Sub",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 19,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::ops::Sub`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 7,
                                },
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
                            ident: Identifier(
                                "bool",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 0,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 0,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::basic::bool`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::basic::bool`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "never",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 1,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 1,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::basic::never`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::basic::never`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "unit",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 2,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 2,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::basic::unit`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::basic::unit`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Trait",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 3,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 3,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::basic::Trait`, `Structure`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 3,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::basic::Trait`, `Structure`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Module",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 4,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 4,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::basic::Module`, `Structure`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 4,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::basic::Module`, `Structure`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                    use_expr_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i8",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 5,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 10,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::i8`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 18,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::i8`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i16",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 6,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 11,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::i16`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 23,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::i16`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 7,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 12,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::i32`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 28,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::i32`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "i64",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 8,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 13,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::i64`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 33,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::i64`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f8",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 9,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 14,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::f8`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 38,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::f8`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f16",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 10,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 15,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::f16`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 43,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::f16`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 11,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 16,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::f32`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 48,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::f32`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "f64",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 12,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 17,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::num::f64`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 53,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::num::f64`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                    use_expr_idx: 3,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "r32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 13,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 20,
                                            path: ModuleItemPath::Type(
                                                TypePath(`core::raw_bits::r32`, `Alien`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`core::raw_bits::r32`, `Alien`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 2,
                                    use_expr_idx: 6,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Debug",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 14,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 21,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::fmt::Debug`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::fmt::Debug`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 3,
                                    use_expr_idx: 9,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Copy",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 15,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 23,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::marker::Copy`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::marker::Copy`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 4,
                                    use_expr_idx: 12,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Sized",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 16,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 24,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::marker::Sized`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::marker::Sized`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 4,
                                    use_expr_idx: 12,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Clone",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 17,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 22,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::clone::Clone`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::clone::Clone`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 5,
                                    use_expr_idx: 15,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "PartialEq",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 18,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 25,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::cmp::PartialEq`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 0,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::cmp::PartialEq`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Eq",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 19,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 26,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::cmp::Eq`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 1,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::cmp::Eq`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "PartialOrd",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 20,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 27,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::cmp::PartialOrd`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 2,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::cmp::PartialOrd`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Ord",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: Use(
                                UseSymbol {
                                    [salsa id]: 21,
                                    original_symbol: ModuleItem(
                                        ModuleItemSymbol {
                                            [salsa id]: 28,
                                            path: ModuleItemPath::Trait(
                                                TraitPath(`core::cmp::Ord`),
                                            ),
                                            accessibility: Accessibility::Public,
                                            ast_idx: 3,
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Trait(
                                            TraitPath(`core::cmp::Ord`),
                                        ),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 6,
                                    use_expr_idx: 18,
                                },
                            ),
                        },
                    ],
                ),
                impl_blocks: [],
                use_expr_rules: UseExprRules(
                    [
                        UseTracker {
                            ast_idx: 0,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 1,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 2,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 3,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 4,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 5,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 6,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Crate(
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
                            state: Resolved {
                                original_symbol: CrateRoot(
                                    ModulePath(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 0,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 1,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 1,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 4,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 4,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 2,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 7,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 3,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 8,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 4,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 5,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 9,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 9,
                                        },
                                    ),
                                ),
                            },
                        },
                        UseTracker {
                            ast_idx: 6,
                            accessibility: Done {
                                accessibility: Accessibility::Public,
                            },
                            variant: Parent {
                                parent_name_token: Identifier(
                                    IdentifierToken {
                                        ident: Identifier(
                                            Word(
                                                Id {
                                                    value: 12,
                                                },
                                            ),
                                        ),
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
                            state: Resolved {
                                original_symbol: Submodule(
                                    SubmoduleSymbol(
                                        Id {
                                            value: 12,
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
                                    value: 5,
                                },
                            ),
                            ast_idx: 1,
                            use_expr_idx: 3,
                            accessibility: Public,
                            progress: 8,
                        },
                        UseAllRule {
                            parent: ModulePath(
                                Id {
                                    value: 8,
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
                                    value: 9,
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
                                    value: 11,
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
                                    value: 10,
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
                                    value: 13,
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
                            ident: Identifier(
                                "r32",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 20,
                                    path: ModuleItemPath::Type(
                                        TypePath(`core::raw_bits::r32`, `Alien`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 2,
                                },
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
                            ident: Identifier(
                                "Debug",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 21,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::fmt::Debug`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
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
                            ident: Identifier(
                                "Clone",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 22,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::clone::Clone`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
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
                            ident: Identifier(
                                "Copy",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 23,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::marker::Copy`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Sized",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 24,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::marker::Sized`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
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
                            ident: Identifier(
                                "PartialEq",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 25,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::cmp::PartialEq`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 0,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Eq",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 26,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::cmp::Eq`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 1,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "PartialOrd",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 27,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::cmp::PartialOrd`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 2,
                                },
                            ),
                        },
                        EntitySymbolEntry {
                            ident: Identifier(
                                "Ord",
                            ),
                            accessibility: Accessibility::Public,
                            symbol: ModuleItem(
                                ModuleItemSymbol {
                                    [salsa id]: 28,
                                    path: ModuleItemPath::Trait(
                                        TraitPath(`core::cmp::Ord`),
                                    ),
                                    accessibility: Accessibility::Public,
                                    ast_idx: 3,
                                },
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
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 32,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            25,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 11,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 44,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            83,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 12,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 45,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            141,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 13,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 46,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            198,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 14,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 47,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            256,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 15,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 48,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            314,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 16,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 49,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            372,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 17,
                                },
                            ),
                        ),
                    ),
                },
                Root {
                    ident_token: IdentifierToken {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 50,
                                },
                            ),
                        ),
                        token_idx: TokenIdx(
                            429,
                        ),
                    },
                    entity_path: ModuleItem(
                        Type(
                            TypePath(
                                Id {
                                    value: 18,
                                },
                            ),
                        ),
                    ),
                },
            ],
        },
        impl_blocks: [],
    },
)