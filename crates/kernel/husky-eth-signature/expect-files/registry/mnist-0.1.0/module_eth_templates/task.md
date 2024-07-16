```rust
[
    (
        ItemPath(`mnist::Task`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::TypeVar(
                        MajorTypeVarEthTemplate {
                            path: MajorFormPath(`mnist::Task`, `TypeVar`),
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
                            path: MajorFormPath(`mnist::TASK`, `StaticVar`),
                            return_ty: EthTerm(`MnistTask`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```