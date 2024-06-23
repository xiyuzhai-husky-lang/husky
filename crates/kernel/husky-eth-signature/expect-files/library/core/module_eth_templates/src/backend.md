```rust
[
    (
        ItemPath(`core::backend::IsBackend`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::backend::IsBackend`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`Self`),
                                    traits: [],
                                },
                            ],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::backend::MlBackend`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::backend::MlBackend`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]
```