```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::Task`, `TypeVar`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::TASK`, `StaticVar`),
            ),
        ),
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