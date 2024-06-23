```rust
[
    (
        ItemPath(`core::cmp::PartialEq`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::cmp::PartialEq`),
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
        ItemPath(`core::cmp::Eq`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::cmp::Eq`),
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
        ItemPath(`core::cmp::PartialOrd`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::cmp::PartialOrd`),
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
        ItemPath(`core::cmp::Ord`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Trait(
                    TraitEthTemplate {
                        path: TraitPath(`core::cmp::Ord`),
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
]
```