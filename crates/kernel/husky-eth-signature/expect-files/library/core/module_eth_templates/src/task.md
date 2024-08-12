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
                                    variable: EthSymbolicVariable(`Self`, `nil`),
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
                            path: MajorFormPath(`core::task::Task`, `TypeVar`),
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
                            path: MajorFormPath(`core::task::TASK`, `StaticVar`),
                            return_ty: EthTerm(`Task`),
                            expr_ty: EthTerm(`Leash Task`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```