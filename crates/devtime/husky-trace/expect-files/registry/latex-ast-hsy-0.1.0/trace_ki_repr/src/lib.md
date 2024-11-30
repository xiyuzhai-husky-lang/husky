```rust
[
    (
        Trace {
            path: TracePath {
                data: TracePathData::StaticVarItem(
                    StaticVarTracePathData {
                        static_var_item_path: MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
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
                                value: 2,
                            },
                        ),
                    ),
                },
            ),
        },
        Some(
            KiRepr {
                ki_domain_repr: Omni,
                opn: KiOpn::Linket(
                    Linket {
                        data: LinketData::MajorStaticVar {
                            path: MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
                            instantiation: LinInstantiation {
                                path: ItemPath(`latex_ast_hsy::AST`),
                                context: LinTypeContext {
                                    comptime_var_overrides: [],
                                },
                                variable_resolutions: [],
                                separator: None,
                            },
                        },
                    },
                ),
                arguments: [],
                source: KiReprSource::Val(
                    MajorFormPath(`latex_ast_hsy::AST`, `StaticVar`),
                ),
                caching_class: Val,
            },
        ),
    ),
]
```