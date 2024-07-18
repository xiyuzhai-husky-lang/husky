```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::StaticVarItem(
                    StaticVarTracePathData {
                        static_var_item_path: MajorFormPath(`mnist::INPUT`, `StaticVar`),
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
                            path: MajorFormPath(`mnist::INPUT`, `StaticVar`),
                            instantiation: LinInstantiation {
                                context: LinTypeContext {
                                    comptime_var_overrides: [],
                                },
                                symbol_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                source: KiReprSource::Val(
                    MajorFormPath(`mnist::INPUT`, `StaticVar`),
                ),
                caching_class: Val,
            },
        ),
    ),
]
```