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
                            ident: `row_start`,
                            ty: DeclarativeTerm(`core::num::i32`),
                        },
                        RegularStructFieldSignature {
                            ident: `row_end`,
                            ty: DeclarativeTerm(`core::num::i32`),
                        },
                        RegularStructFieldSignature {
                            ident: `upper_mass`,
                            ty: DeclarativeTerm(`core::num::i32`),
                        },
                        RegularStructFieldSignature {
                            ident: `lower_mass`,
                            ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ],
                },
            ),
        ),
    ),
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
                            ty: DeclarativeTerm(`[] core::option::Option ~ mnist_classifier::raw_contour::RawContour`),
                        },
                    ],
                },
            ),
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignatureTemplate::Fn(
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
                                            value: 29,
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::option::Option core::num::f32`),
                },
            ),
        ),
    ),
    Err(
        FieldTypeDeclarativeTermError(
            0,
        ),
    ),
    Ok(
        Signature::Form(
            FormDeclarativeSignatureTemplate::Fn(
                FnDeclarativeSignatureTemplate {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatureTemplates {
                        parameters: [
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            ExplicitParameterSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 56,
                                            },
                                        ),
                                    ),
                                ),
                            },
                        ],
                    },
                    return_ty: DeclarativeTerm(`core::raw_bits::r32`),
                },
            ),
        ),
    ),
    Err(
        ParameterTypeDeclarativeTermError(
            0,
        ),
    ),
    Ok(
        Signature::ImplBlock(
            ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignature {
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
                    TypeMemoizedFieldDeclarativeSignature {
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
                    TypeMemoizedFieldDeclarativeSignature {
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
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MemoizedField(
                    TypeMemoizedFieldDeclarativeSignature {
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
                    TypeMemoizedFieldDeclarativeSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                TypeItemDeclarativeSignatureTemplate::MethodFn(
                    TypeMethodFnDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates {
                            data: [],
                        },
                        self_parameter: ExplicitParameterSignature {
                            contract: Pure,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 72,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                            parameters: [
                                ExplicitParameterSignature {
                                    contract: Pure,
                                    ty: EntityPath(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    ),
                                },
                            ],
                        },
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
]