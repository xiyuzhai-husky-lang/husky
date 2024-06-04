```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::Task`, `TypeAlias`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::TypeAlias(
                        MajorTypeAliasEthTemplate {
                            path: FormPath(`mnist::Task`, `TypeAlias`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`mnist::TASK`, `Static`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Static(
                        MajorStaticEthTemplate {
                            path: FormPath(`mnist::TASK`, `Static`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```