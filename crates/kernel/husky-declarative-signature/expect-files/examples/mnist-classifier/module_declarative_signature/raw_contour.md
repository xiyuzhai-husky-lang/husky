[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `cc`,
                                    ty: DeclarativeTerm(`core::mem::Ref mnist_classifier::connected_component::ConnectedComponent`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `points`,
                                    ty: DeclarativeTerm(`[] mnist_classifier::geom2d::Point2d`),
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
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
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
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
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
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
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
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 80,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 80,
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
                FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 64,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 80,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::raw_contour::Direction`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `prev1`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `prev2`,
                                    ty: DeclarativeTerm(`core::num::i32`),
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
                FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 71,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
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
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: ExplicitApplication(
                                            DeclarativeTermExplicitApplication(
                                                Id {
                                                    value: 69,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
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
                        value: 34,
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
                        ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 79,
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
                        value: 32,
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
                        ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `line_segment_sketch`,
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
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `relative_bounding_box`,
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
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `contour_len`,
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
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
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
                        module_path: `mnist_classifier::raw_contour`,
                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::raw_contour::RawContour`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 79,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 50,
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
]