[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`core::clone::Clone`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameters {
                            data: [],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
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
                        generic_parameters: DeclarativeGenericParameters {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::clone::Clone`),
                        self_ty: DerivedAny(
                            DeclarativeTermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
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
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: Symbol(
                                    DeclarativeTermSymbol(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
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