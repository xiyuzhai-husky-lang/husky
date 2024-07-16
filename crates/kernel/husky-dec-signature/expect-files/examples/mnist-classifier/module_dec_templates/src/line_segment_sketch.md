```rust
[
    (
        ItemPath(`mnist_classifier::line_segment_sketch::concave_component`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::convex_component`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::convexity`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::line_segment`),
        Ok(
            ItemDecTemplate::Submodule,
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `points`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::LeashOrBitNot(
                                                Toolchain {
                                                    data: ToolchainData::Local {
                                                        library_path: "../../../library",
                                                    },
                                                },
                                            ),
                                            argument: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        ),
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `start`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: true,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `end`,
                                    ty: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        ),
                                    ),
                                    has_initialization: true,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::Application(
                                                        DecApplication {
                                                            function: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                            argument: DecTerm::EntityPath(
                                                                DecItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::PropsStruct(
                        PropsStructDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            fields: [
                                PropsStructFieldDecTemplate {
                                    ident: `contour`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::LeashOrBitNot(
                                                Toolchain {
                                                    data: ToolchainData::Local {
                                                        library_path: "../../../library",
                                                    },
                                                },
                                            ),
                                            argument: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                                PropsStructFieldDecTemplate {
                                    ident: `strokes`,
                                    ty: DecTerm::Application(
                                        DecApplication {
                                            function: DecTerm::List(
                                                DecList {
                                                    toolchain: Toolchain {
                                                        data: ToolchainData::Local {
                                                            library_path: "../../../library",
                                                        },
                                                    },
                                                    items: [],
                                                },
                                            ),
                                            argument: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    has_initialization: false,
                                },
                            ],
                            instance_constructor_ritchie_ty: DecRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                params: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::List(
                                                        DecList {
                                                            toolchain: Toolchain {
                                                                data: ToolchainData::Local {
                                                                    library_path: "../../../library",
                                                                },
                                                            },
                                                            items: [],
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                ],
                                return_ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::go_right`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::go_left`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::extend_end`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::extend_start`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::num::i32`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::List(
                                        DecList {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            items: [],
                                        },
                                    ),
                                    argument: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::EntityPath(
                            DecItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::visual::Visual`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::AssocRitchie(
                        TypeAssocRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodRitchie(
                        TypeMethodRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                            },
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        trai: DecTerm::EntityPath(
                            DecItemPath::Trait(
                                TraitPath(`core::visual::Visualize`),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::MethodRitchie(
                        TraitForTypeMethodRitchieDecTemplate {
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`core::visual::Visual`, `Extern`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
        Ok(
            ItemDecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [],
                        },
                        ty: DecTerm::EntityPath(
                            DecItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::Application(
                                DecApplication {
                                    function: DecTerm::List(
                                        DecList {
                                            toolchain: Toolchain {
                                                data: ToolchainData::Local {
                                                    library_path: "../../../library",
                                                },
                                            },
                                            items: [],
                                        },
                                    ),
                                    argument: DecTerm::EntityPath(
                                        DecItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MemoizedField(
                        TypeMemoizedFieldDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [],
                                },
                                ty: DecTerm::EntityPath(
                                    DecItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
        Ok(
            ItemDecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::AssocRitchie(
                        TypeAssocRitchieDecTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                                TypeItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::Application(
                                                DecApplication {
                                                    function: DecTerm::LeashOrBitNot(
                                                        Toolchain {
                                                            data: ToolchainData::Local {
                                                                library_path: "../../../library",
                                                            },
                                                        },
                                                    ),
                                                    argument: DecTerm::EntityPath(
                                                        DecItemPath::Type(
                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: DecTerm::EntityPath(
                                                DecItemPath::Type(
                                                    TypePath(`core::num::f32`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm::EntityPath(
                                DecItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```