[
    (
        EntityPath::Module(
            `mnist_classifier::line_segment_sketch::concave_component`,
        ),
        Ok(
            SignatureTemplate::Module,
        ),
    ),
    (
        EntityPath::Module(
            `mnist_classifier::line_segment_sketch::convex_component`,
        ),
        Ok(
            SignatureTemplate::Module,
        ),
    ),
    (
        EntityPath::Module(
            `mnist_classifier::line_segment_sketch::convexity`,
        ),
        Ok(
            SignatureTemplate::Module,
        ),
    ),
    (
        EntityPath::Module(
            `mnist_classifier::line_segment_sketch::line_segment`,
        ),
        Ok(
            SignatureTemplate::Module,
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            ),
        ),
        Err(
            FieldTypeDeclarativeTermError(
                0,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `contour`,
                                    ty: DeclarativeTerm(`~ mnist_classifier::raw_contour::RawContour`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `strokes`,
                                    ty: DeclarativeTerm(`[] mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                },
                            ],
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 85,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::ImplBlock(
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 35,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `visualize`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 89,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::basic::unit`),
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
                        value: 38,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
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
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `new`,
                    item_kind: AssociatedFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 51,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 89,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
        EntityPath::ImplBlock(
            TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(
                    Id {
                        value: 36,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `visualize`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(
                    TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(
                        TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignature {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 90,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
        EntityPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 39,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
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
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `concave_components`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `bounding_box`,
                    item_kind: MemoizedField,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MemoizedField(
                        TypeMemoizedFieldDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                implicit_parameters: ImplicitParameterDeclarativeSignatures {
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `new`,
                    item_kind: AssociatedFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            self_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 44,
                                                },
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignature {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 61,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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