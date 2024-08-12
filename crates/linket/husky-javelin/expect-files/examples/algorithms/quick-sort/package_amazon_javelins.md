```rust
[
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::Form(
                    MajorFormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                        Fn,
                    )`),
                ),
                instantiation: JavInstantiation {
                    path: ItemPath(`quick_sort::quick_sort_works_for_integers`),
                    context: JavTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::Form(
                    MajorFormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                        Fn,
                    )`),
                ),
                instantiation: JavInstantiation {
                    path: ItemPath(`quick_sort::quick_sort_works_for_strs`),
                    context: JavTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
]
```