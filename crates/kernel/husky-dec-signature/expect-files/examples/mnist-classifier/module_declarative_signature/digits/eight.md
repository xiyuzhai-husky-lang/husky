[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm(`mnist_classifier::fermi::FermiMatchResult`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: DecTerm(`malamute::OneVsAll mnist::MnistLabel mnist::MnistLabel::Eight`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Fn(
                        MajorFnDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`~ mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`core::option::Option core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]