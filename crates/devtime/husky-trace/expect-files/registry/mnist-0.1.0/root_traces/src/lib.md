```rust
[
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
]
```