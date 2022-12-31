Ok(
    EntityTreePresheet {
        module_path: `core::num`,
        module_symbols: [
            ModuleItem {
                ident: `i8`,
                accessibility: Public,
                ast_idx: 9,
                path: TypePath(`core::num::i8`, `Foreign`),
            },
            ModuleItem {
                ident: `i16`,
                accessibility: Public,
                ast_idx: 13,
                path: TypePath(`core::num::i16`, `Foreign`),
            },
            ModuleItem {
                ident: `i32`,
                accessibility: Public,
                ast_idx: 17,
                path: TypePath(`core::num::i32`, `Foreign`),
            },
            ModuleItem {
                ident: `i64`,
                accessibility: PubicUnder(
                    `core::num`,
                ),
                ast_idx: 21,
                path: TypePath(`core::num::i64`, `Foreign`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)