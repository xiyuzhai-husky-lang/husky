```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::task::IsTask`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`core::task::Task`, `TypeVar`),
            ),
        ),
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`core::task::TASK`, `Static`),
            ),
        ),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Static(
                        MajorStaticEthTemplate {
                            path: FormPath(`core::task::TASK`, `Static`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```