[
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`[] mnist_classifier::connected_component::ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`~ mnist_classifier::connected_component::ConnectedComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_raw_contours`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`~ [] mnist_classifier::raw_contour::RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_raw_contour`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`~ mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajarItemPath::Fugitive(
                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Val(
                        ValFugitiveDeclarativeSignatureTemplate {
                            initialization_ty: DeclarativeTerm(`~ [] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
                        },
                    ),
                ),
            ),
        ),
    ),
]