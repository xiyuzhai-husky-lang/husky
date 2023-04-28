[
    Err(
        FieldTypeDeclarativeTermError(
            1,
        ),
    ),
    Ok(
        SignatureTemplate::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                TraitForTypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    trai: DeclarativeTerm(`core::visual::Visualize`),
                    ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::convex_component::ConvexComponent`),
                },
            ),
        ),
    ),
    Ok(
        SignatureTemplate::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                    TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 87,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        return_ty: DeclarativeTerm(`core::visual::Html`),
                    },
                ),
            ),
        ),
    ),
]