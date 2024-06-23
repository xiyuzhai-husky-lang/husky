```rust
[
    (
        ItemPath(`mnist::task::MnistTask`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`mnist::task::MnistTask`, `Extern`),
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
        ItemPath(`mnist::task::MnistTask(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist::task::MnistTask(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`MnistTask`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::task::MnistTask(0)::new`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    AssocRitchie(
                        TypeAssocRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```