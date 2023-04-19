[
    Ok(
        Signature::Type(
            TypeDeclarativeSignatureTemplate::RegularStruct(
                RegularStructTypeDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    fields: [
                        RegularStructFieldSignature {
                            ident: `matches`,
                            ty: DeclarativeTerm(`[] core::option::Option ~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                        RegularStructFieldSignature {
                            ident: `others`,
                            ty: DeclarativeTerm(`[] ~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FugitiveDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 41,
                                        },
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: ExplicitApplicationOrRitchieCall(
                                    DeclarativeTermExplicitApplicationOrRitchieCall(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::fermi::FermiMatchResult`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignatureTemplate {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignatureTemplate {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignatureTemplate {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]