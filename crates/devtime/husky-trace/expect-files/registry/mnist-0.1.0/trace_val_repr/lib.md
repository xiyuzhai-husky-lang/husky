[
    (
        TraceId {
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
                            value: 538,
                        },
                    ),
                    val_item_path: FugitivePath(
                        ItemPathId(
                            Id {
                                value: 497,
                            },
                        ),
                    ),
                },
            ),
        },
        Some(
            ValRepr {
                val_domain_repr: Omni,
                opn: ValOpn::ValItem(
                    FugitivePath(`mnist::input`, `Val`),
                ),
                arguments: [],
                caching_class: ValItem,
            },
        ),
    ),
]