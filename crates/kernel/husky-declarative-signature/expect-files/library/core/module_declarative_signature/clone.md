[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::clone::Clone`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 17,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::clone::Clone`),
                        self_ty: DerivedAny(
                            DeclarativeTermSymbol(
                                Id {
                                    value: 3,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::clone`,
                        trai_path: TraitPath(`core::clone::Clone`),
                        ty_sketch: DeriveAny,
                        disambiguator: 0,
                    },
                    ident: `clone`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`t`),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]