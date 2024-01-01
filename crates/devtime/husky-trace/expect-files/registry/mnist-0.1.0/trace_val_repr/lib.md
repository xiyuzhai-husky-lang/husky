[
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FugitivePath(`mnist::input`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 488,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 505,
                            },
                        ),
                    ),
                },
            ),
        },
        Some(
            ValRepr {
                val_domain_repr: Omni,
                opn: ValOpn::Linkage(
                    Linkage {
                        data: LinkageData::ValItem {
                            path: FugitivePath(`mnist::input`, `Val`),
                            instantiation: LinkageInstantiation {
                                symbol_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
    ),
]