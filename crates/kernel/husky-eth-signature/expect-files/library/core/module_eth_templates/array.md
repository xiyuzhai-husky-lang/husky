```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::array::Array`, `Extern`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::array::Array`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`L`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: EthSymbolicVariable(`E`),
                                        traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]
```