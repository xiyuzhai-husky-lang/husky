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
        ItemPath(`mnist::task`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::StaticVar(
                        MajorStaticVarEthTemplate {
                            path: FormPath(`mnist::task`, `StaticVar`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```