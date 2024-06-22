```rust
[
    (
        ItemPath(`mnist::Task`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::TypeVar(
                        MajorTypeVarEthTemplate {
                            path: FormPath(`mnist::Task`, `TypeVar`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::TASK`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::StaticVar(
                        MajorStaticVarEthTemplate {
                            path: FormPath(`mnist::TASK`, `StaticVar`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```