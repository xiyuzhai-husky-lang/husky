[
    Ok(
        Signature::Type(
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
            TypeDeclarativeSignature::RegularStruct(
                RegularStructTypeDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
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
            FormDeclarativeSignature::Fn(
                FnDeclarativeSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    parameters: ExplicitParameterDeclarativeSignatures {
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
        Signature::Impl(
            ImplBlockDeclarativeSignature::TypeImpl(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterDeclarativeSignatures {
                        data: [],
                    },
                    ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::Memo(
                    TypeMemoSignature {
                        return_ty: DeclarativeTerm(`core::num::f32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::MethodFn(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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
            AssociatedItemDeclarativeSignature::TypeItem(
                TypeItemDeclarativeSignature::MethodFn(
                    TypeMethodSignature {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
                        nonself_regular_parameters: ExplicitParameterDeclarativeSignatures {
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