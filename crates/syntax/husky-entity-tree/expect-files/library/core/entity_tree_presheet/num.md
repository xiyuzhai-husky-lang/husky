Ok(
    EntityTreePresheet {
        module_path: `core::num`,
        module_symbols: [
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
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)