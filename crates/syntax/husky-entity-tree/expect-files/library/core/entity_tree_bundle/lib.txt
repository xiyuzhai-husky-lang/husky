Ok(
    EntityTreeBundle {
        sheets: [
            EntityTreeSheet {
                module_path: `core`,
                sheets: [
                    Module {
                        ident: `basic`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        module_path: `core::basic`,
                    },
                    Module {
                        ident: `logic`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        module_path: `core::logic`,
                    },
                    Module {
                        ident: `num`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        module_path: `core::num`,
                    },
                    Module {
                        ident: `ops`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        module_path: `core::ops`,
                    },
                    Module {
                        ident: `prelude`,
                        accessibility: PubicUnder(
                            `core`,
                        ),
                        module_path: `core::prelude`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::basic`,
                sheets: [
                    ModuleItem {
                        ident: `bool`,
                        accessibility: Public,
                        ast_idx: 0,
                        path: `core::basic::bool`,
                    },
                    ModuleItem {
                        ident: `Trait`,
                        accessibility: Public,
                        ast_idx: 1,
                        path: `core::basic::Trait`,
                    },
                    ModuleItem {
                        ident: `Module`,
                        accessibility: Public,
                        ast_idx: 2,
                        path: `core::basic::Module`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::logic`,
                sheets: [
                    ModuleItem {
                        ident: `LogicAnd`,
                        accessibility: Public,
                        ast_idx: 0,
                        path: `core::logic::LogicAnd`,
                    },
                    ModuleItem {
                        ident: `LogicOr`,
                        accessibility: Public,
                        ast_idx: 1,
                        path: `core::logic::LogicOr`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::num`,
                sheets: [
                    ModuleItem {
                        ident: `i8`,
                        accessibility: Public,
                        ast_idx: 9,
                        path: `core::num::i8`,
                    },
                    ModuleItem {
                        ident: `i16`,
                        accessibility: Public,
                        ast_idx: 13,
                        path: `core::num::i16`,
                    },
                    ModuleItem {
                        ident: `i32`,
                        accessibility: Public,
                        ast_idx: 17,
                        path: `core::num::i32`,
                    },
                    ModuleItem {
                        ident: `i64`,
                        accessibility: PubicUnder(
                            `core::num`,
                        ),
                        ast_idx: 21,
                        path: `core::num::i64`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::ops`,
                sheets: [
                    ModuleItem {
                        ident: `Output`,
                        accessibility: PubicUnder(
                            `core::ops`,
                        ),
                        ast_idx: 0,
                        path: `core::ops::Output`,
                    },
                    ModuleItem {
                        ident: `Add`,
                        accessibility: Public,
                        ast_idx: 3,
                        path: `core::ops::Add`,
                    },
                ],
            },
            EntityTreeSheet {
                module_path: `core::prelude`,
                sheets: [],
            },
        ],
    },
)