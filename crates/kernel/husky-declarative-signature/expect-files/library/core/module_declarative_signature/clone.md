[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::clone::Clone`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Trait(
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
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `core::clone`,
                    trai_path: TraitPath(`core::clone::Clone`),
                    ty_sketch: TypeSketch::DeriveAny,
                    disambiguator: 0,
                },
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
                        ty_sketch: TypeSketch::DeriveAny,
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
                            self_ty: DeclarativeTerm(`t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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