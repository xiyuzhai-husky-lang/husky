Ok(
    EntityTreePresheet {
        module_path: `quick_sort`,
        module_symbols: [
            ModuleItem {
                ident: `quick_sort`,
                accessibility: Public,
                ast_idx: 30,
                path: `quick_sort::quick_sort`,
            },
            ModuleItem {
                ident: `quick_sort_aux`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 31,
                path: `quick_sort::quick_sort_aux`,
            },
            ModuleItem {
                ident: `partition`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 32,
                path: `quick_sort::partition`,
            },
            ModuleItem {
                ident: `quick_sort_works_for_integers`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 34,
                path: `quick_sort::quick_sort_works_for_integers`,
            },
            ModuleItem {
                ident: `quick_sort_works_for_strs`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 36,
                path: `quick_sort::quick_sort_works_for_strs`,
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)