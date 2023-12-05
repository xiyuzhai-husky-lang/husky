[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::FunctionFn(
                        FnFugitiveDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::bool`),
                        },
                    ),
                ),
            ),
        ),
    ),
]