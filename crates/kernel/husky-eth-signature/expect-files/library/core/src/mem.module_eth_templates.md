```rust
[
    (
        ItemPath(`core::mem::Ref`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::mem::Ref`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`'a`, `mono`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
    (
        ItemPath(`core::mem::RefMut`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::mem::RefMut`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`'a`, `mono`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Invariant,
                                        ),
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
    (
        ItemPath(`core::mem::Leash`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::mem::Leash`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
    (
        ItemPath(`core::mem::At`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::mem::At`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'α`, `mono`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
    (
        ItemPath(`core::mem::Leash as core::marker::Copy(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::mem::Leash as core::marker::Copy(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`E`, `mono`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Copy`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
]
```