Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `quick_sort`,
                module_symbols: [
                    ModuleItem {
                        ident: `quick_sort`,
                        accessibility: Public,
                        ast_idx: 30,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `quick_sort`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `quick_sort_aux`,
                        accessibility: PubicUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 31,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `quick_sort`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `partition`,
                        accessibility: PubicUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 32,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `quick_sort`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 50,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `quick_sort_works_for_integers`,
                        accessibility: PubicUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 34,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `quick_sort`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 58,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                    ModuleItem {
                        ident: `quick_sort_works_for_strs`,
                        accessibility: PubicUnder(
                            `quick_sort`,
                        ),
                        ast_idx: 36,
                        path: Connected(
                            ConnectedModuleItemPath {
                                module_path: `quick_sort`,
                                ident: Identifier(
                                    Word(
                                        Id {
                                            value: 60,
                                        },
                                    ),
                                ),
                            },
                        ),
                    },
                ],
            },
        ],
    },
)