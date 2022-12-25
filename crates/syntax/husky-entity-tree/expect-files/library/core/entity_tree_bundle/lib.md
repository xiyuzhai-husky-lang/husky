Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `core`,
                module_symbols: [
                    Module {
                        ident: `basic`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        ast_idx: 0,
                        module_path: `core::basic`,
                    },
                    Module {
                        ident: `logic`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        ast_idx: 1,
                        module_path: `core::logic`,
                    },
                    Module {
                        ident: `num`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        ast_idx: 2,
                        module_path: `core::num`,
                    },
                    Module {
                        ident: `ops`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        ast_idx: 3,
                        module_path: `core::ops`,
                    },
                    Module {
                        ident: `prelude`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        ast_idx: 4,
                        module_path: `core::prelude`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::basic`,
                module_symbols: [
                    ModuleItem {
                        ident: `bool`,
                        accessibility: Public,
                        ast_idx: 0,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::basic`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `Trait`,
                        accessibility: Public,
                        ast_idx: 1,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::basic`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 7,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `Module`,
                        accessibility: Public,
                        ast_idx: 2,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::basic`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 8,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::logic`,
                module_symbols: [
                    ModuleItem {
                        ident: `LogicAnd`,
                        accessibility: Public,
                        ast_idx: 0,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::logic`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 9,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `LogicOr`,
                        accessibility: Public,
                        ast_idx: 1,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::logic`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 15,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::num`,
                module_symbols: [
                    ModuleItem {
                        ident: `i8`,
                        accessibility: Public,
                        ast_idx: 9,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::num`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 19,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `i16`,
                        accessibility: Public,
                        ast_idx: 13,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::num`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 26,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `i32`,
                        accessibility: Public,
                        ast_idx: 17,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::num`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 27,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `i64`,
                        accessibility: PubicUnder(
                            `core::num`,
                        ),
                        ast_idx: 21,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::num`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 28,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::ops`,
                module_symbols: [
                    ModuleItem {
                        ident: `Add`,
                        accessibility: Public,
                        ast_idx: 3,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `core::ops`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 22,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::prelude`,
                module_symbols: [],
            },
        ],
    },
)