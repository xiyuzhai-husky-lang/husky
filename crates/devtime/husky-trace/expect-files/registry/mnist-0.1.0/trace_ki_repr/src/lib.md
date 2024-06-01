```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::ValItem(
                    ValItemTracePathData {
                        val_item_path: FormPath(`mnist::input`, `Val`),
                    },
                ),
            },
            data: ValItem(
                ValItemTraceData {
                    path: TracePath(
                        Id {
                            value: 1,
                        },
                    ),
                    val_item_path: MajorFormPath(
                        ItemPathId(
                            Id {
                                value: 15,
                            },
                        ),
                    ),
                },
            ),
        },
        Some(
            KiRepr {
                ki_domain_repr: Omni,
                opn: KiOpn::Linkage(
                    Linkage {
                        data: LinkageData::MajorVal {
                            path: FormPath(`mnist::input`, `Val`),
                            instantiation: LinInstantiation {
                                symbol_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                source: KiReprSource::ValItem(
                    FormPath(`mnist::input`, `Val`),
                ),
                caching_class: ValItem,
            },
        ),
    ),
]
```