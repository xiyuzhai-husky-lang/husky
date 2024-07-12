```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::StaticVarItem(
                    StaticVarTracePathData {
                        static_var_item_path: FormPath(`mnist::INPUT`, `StaticVar`),
                    },
                ),
            },
            data: StaticVar(
                StaticVarTraceData {
                    path: TracePath(
                        Id {
                            value: 1,
                        },
                    ),
                    static_var_item_path: MajorFormPath(
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
                        data: LinkageData::MajorStaticVar {
                            path: FormPath(`mnist::INPUT`, `StaticVar`),
                            instantiation: LinInstantiation {
                                symbol_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                source: KiReprSource::ValItem(
                    FormPath(`mnist::INPUT`, `StaticVar`),
                ),
                caching_class: Val,
            },
        ),
    ),
]
```