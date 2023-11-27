[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `start`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `end`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                    has_initialization: false,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId(
                        Id {
                            value: 331,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 386,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(
                    ItemPathId(
                        Id {
                            value: 387,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 387,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
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