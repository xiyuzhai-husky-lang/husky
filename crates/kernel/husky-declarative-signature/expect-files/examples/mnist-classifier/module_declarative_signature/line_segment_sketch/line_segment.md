[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `start`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `end`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 45,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `displacement`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                generic_parameters: DeclarativeGenericParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 95,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `dist_to_point`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                generic_parameters: DeclarativeGenericParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 95,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]