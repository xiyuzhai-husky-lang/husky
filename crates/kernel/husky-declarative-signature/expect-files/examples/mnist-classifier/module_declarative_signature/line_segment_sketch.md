[
    (
        ItemPath::Submodule(
            Room32,
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::concave_component`,
            ),
        ),
        Ok(
            SignatureTemplate::Submodule,
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convex_component`,
            ),
        ),
        Ok(
            SignatureTemplate::Submodule,
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::convexity`,
            ),
        ),
        Ok(
            SignatureTemplate::Submodule,
        ),
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmodulePath(
                `mnist_classifier::line_segment_sketch::line_segment`,
            ),
        ),
        Ok(
            SignatureTemplate::Submodule,
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `points`,
                                    ty: DeclarativeTerm(`~ core::slice::CyclicSlice mnist_classifier::geom2d::Point2d`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `start`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                    has_initialization: true,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `end`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                                    has_initialization: true,
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `contour`,
                                    ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                    has_initialization: false,
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `strokes`,
                                    ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
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
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
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
                                            contract: None,
                                            ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
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
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
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
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
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
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: Path(
                            EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 216,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId(
                        Id {
                            value: 388,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::visual::Html`),
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
                            value: 325,
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
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
                            value: 389,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 389,
                                    },
                                ),
                            ),
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
                            value: 390,
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
                                        value: 390,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: Path(
                            EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 217,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId(
                        Id {
                            value: 391,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::visual::Html`),
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
                            value: 327,
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
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
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
                            value: 392,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                            value: 393,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 393,
                                    },
                                ),
                            ),
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                            value: 394,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            path: TypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 394,
                                    },
                                ),
                            ),
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                        },
                    ),
                ),
            ),
        ),
    ),
]