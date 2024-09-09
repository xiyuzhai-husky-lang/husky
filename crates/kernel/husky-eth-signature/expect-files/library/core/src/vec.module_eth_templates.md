```rust
[
    (
        ItemPath(`core::vec::Vec`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::vec::Vec`, `Extern`),
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
        ItemPath(`core::vec::Vec(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::vec::Vec(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`E`, `mono`),
                                    traits: [],
                                },
                            ],
                        },
                        self_ty: EthTerm(`Vec E`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::ilen`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::push`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::push`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_lifetime`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`E`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::first`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_place`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::At,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Option At 'self_place E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::last`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_place`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::At,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Option At 'self_place E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::pop`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::pop`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_lifetime`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Option E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::collect_leashes`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::collect_leashes`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Leash,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Vec Leash E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::cyclic_slice_leashed`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Leash,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`Leash CyclicSlice E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::vec::Vec(0)::pop_with_largest_opt_f32`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::vec::Vec(0)::pop_with_largest_opt_f32`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Vec E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_lifetime`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: EthTerm(`Vec E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`fn(( E) -> Option f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`Option E`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```