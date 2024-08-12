```rust
[
    (
        ItemPath(`mnist::task`),
        Ok(
            ItemEthTemplate::Submodule(
                SubmoduleItemPath(`mnist::task),
            ),
        ),
    ),
    (
        ItemPath(`mnist::MnistLabel`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`mnist::MnistLabel`, `Enum`),
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
        ItemPath(`mnist::BinaryImage28`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`mnist::BinaryImage28`, `Extern`),
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
        ItemPath(`mnist::BinaryGrid28`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`mnist::BinaryGrid28`, `Extern`),
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
        ItemPath(`mnist::INPUT`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::StaticVar(
                        MajorStaticVarEthTemplate {
                            path: MajorFormPath(`mnist::INPUT`, `StaticVar`),
                            return_ty: EthTerm(`BinaryImage28`),
                            expr_ty: EthTerm(`Leash BinaryImage28`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryImage28 as core::visual::Visualize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::visual::Visualize(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Visualize`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist::BinaryImage28 as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist::BinaryImage28 as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`BinaryImage28`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`BinaryImage28`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Visual`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryImage28(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist::BinaryImage28(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`BinaryImage28`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryImage28(0)::new_zeros`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::AssocRitchie(
                        TypeAssocRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist::BinaryImage28(0)::new_zeros`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`BinaryImage28`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`BinaryImage28`),
                            ty: EthTerm(`fn(() -> BinaryImage28`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryImage28 as core::ops::IntIndex(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist::BinaryImage28 as core::ops::IntIndex(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`IntIndex`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 13,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist::BinaryImage28 as core::ops::IntIndex(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocType(
                        TraitForTypeAssocTypeEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist::BinaryImage28 as core::ops::IntIndex(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            assoc_ty: EthTerm(`r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryGrid28 as core::visual::Visualize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::visual::Visualize(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Visualize`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist::BinaryGrid28 as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist::BinaryGrid28 as core::visual::Visualize(0)>::visualize`,
                                TraitItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`BinaryGrid28`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`BinaryGrid28`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Visual`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryGrid28(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist::BinaryGrid28(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`BinaryGrid28`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryGrid28(0)::new_zeros`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::AssocRitchie(
                        TypeAssocRitchieEthTemplate {
                            path: TypeItemPath(
                                `mnist::BinaryGrid28(0)::new_zeros`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`BinaryGrid28`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`BinaryGrid28`),
                            ty: EthTerm(`fn(() -> BinaryGrid28`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist::BinaryGrid28 as core::ops::IntIndex(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`mnist::BinaryGrid28 as core::ops::IntIndex(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`IntIndex`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 14,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist::BinaryGrid28 as core::ops::IntIndex(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocType(
                        TraitForTypeAssocTypeEthTemplate {
                            path: TraitForTypeItemPath(
                                `<mnist::BinaryGrid28 as core::ops::IntIndex(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            assoc_ty: EthTerm(`r32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```