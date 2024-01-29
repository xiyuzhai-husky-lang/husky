[
    Trace {
        path: TracePath {
            data: TracePathData::Submodule(
                SubmoduleTracePathData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::digits`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        data: Submodule(
            SubmoduleTraceData {
                submodule_item_path: SubmoduleItemPath(
                    ItemPathId(
                        Id {
                            value: 6,
                        },
                    ),
                ),
            },
        ),
    },
    Trace {
        path: TracePath {
            data: TracePathData::Submodule(
                SubmoduleTracePathData {
                    submodule_item_path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `mnist_classifier::major`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        },
        data: Submodule(
            SubmoduleTraceData {
                submodule_item_path: SubmoduleItemPath(
                    ItemPathId(
                        Id {
                            value: 7,
                        },
                    ),
                ),
            },
        ),
    },
    Trace {
        path: TracePath {
            data: TracePathData::ValItem(
                ValItemTracePathData {
                    val_item_path: FugitivePath(`mnist_classifier::main`, `Val`),
                },
            ),
        },
        data: ValItem(
            ValItemTraceData {
                path: TracePath(
                    Id {
                        value: 3,
                    },
                ),
                val_item_path: FugitivePath(
                    ItemPathId(
                        Id {
                            value: 8,
                        },
                    ),
                ),
            },
        ),
    },
]