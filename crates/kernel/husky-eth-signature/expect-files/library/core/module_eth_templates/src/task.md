```rust
[
    (
        ItemPath(`core::task::IsTask`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::task::IsTask`),
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
        ItemPath(`core::task::Task`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::TypeVar(
                        MajorTypeVarEthTemplate {
                            path: FormPath(`core::task::Task`, `TypeVar`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::task::TASK`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::StaticVar(
                        MajorStaticVarEthTemplate {
                            path: FormPath(`core::task::TASK`, `StaticVar`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```