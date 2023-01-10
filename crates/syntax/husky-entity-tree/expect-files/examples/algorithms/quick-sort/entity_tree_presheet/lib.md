Ok(
    EntityTreePresheet {
        module_path: `quick_sort`,
        module_specific_symbols: [
            ModuleItem {
                ident: `quick_sort`,
                accessibility: Public,
                ast_idx: 30,
                path: FormPath(`quick_sort::quick_sort`, `Function`),
            },
            ModuleItem {
                ident: `quick_sort_aux`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 31,
                path: FormPath(`quick_sort::quick_sort_aux`, `Function`),
            },
            ModuleItem {
                ident: `partition`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 32,
                path: FormPath(`quick_sort::partition`, `Function`),
            },
            ModuleItem {
                ident: `quick_sort_works_for_integers`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 34,
                path: FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
            },
            ModuleItem {
                ident: `quick_sort_works_for_strs`,
                accessibility: PubicUnder(
                    `quick_sort`,
                ),
                ast_idx: 36,
                path: FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [],
        ),
    },
)