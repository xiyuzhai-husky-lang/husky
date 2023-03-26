[
    Ok(
        Signature::Trait(
            TraitSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    data: [],
                },
            },
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::i8`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::i8`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::i16`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 43,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::i16`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::i32`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
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
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::i32`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::i64`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 45,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::i64`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::i128`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 46,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::i128`),
                    },
                ),
            ),
        ),
    ),
    Ok(
        Signature::Impl(
            ImplSignature::TypeAsTraitImpl(
                TraitForTypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    trai: RawTerm(`core::clone::Clone`),
                    ty: RawTerm(`core::num::isize`),
                },
            ),
        ),
    ),
    Ok(
        Signature::AssociatedItem(
            AssociatedItemSignature::TypeAsTraitItem(
                TypeAsTraitItemSignature::MethodFn(
                    TraitForTypeMethodSignature {
                        implicit_parameters: ImplicitParameterSignatures {
                            data: [],
                        },
                        self_parameter: RegularParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 47,
                                        },
                                    ),
                                ),
                            ),
                        },
                        nonself_regular_parameters: RegularParameterSignatures {
                            parameters: [],
                        },
                        return_ty: RawTerm(`core::num::isize`),
                    },
                ),
            ),
        ),
    ),
]