[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `row_start`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `row_end`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `upper_mass`,
                                    ty: DeclarativeTerm(`core::num::i32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `lower_mass`,
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
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `matches`,
                                    ty: DeclarativeTerm(`[] core::option::Option ~ mnist_classifier::raw_contour::RawContour`),
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
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: ExplicitApplication(
                                                DeclarativeTermExplicitApplication(
                                                    Id {
                                                        value: 46,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::PropsStruct(
                        PropsStructDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `mask`,
                                    ty: DeclarativeTerm(`mnist::BinaryImage28`),
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
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 56,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
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
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Fugitive(
                    FugitiveDeclarativeSignatureTemplate::Fn(
                        FnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        Id {
                                                            value: 100,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::connected_component::ConnectedComponent`),
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
                        generic_parameters: DeclarativeGenericParameters {
                            data: [],
                        },
                        trai: DeclarativeTerm(`core::visual::Visualize`),
                        self_ty: Path(
                            EntityPath(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 81,
                                        },
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::connected_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 81,
                                },
                            ),
                        ),
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
                        TraitForTypeMethodFnDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 81,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
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
                        value: 35,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameters {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `raw_contours`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`[] mnist_classifier::raw_contour::RawContour`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `eff_holes`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::connected_component::EffHoles`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `max_hole_ilen`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `max_row_span`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `row_span_sum`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `distribution`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `upper_mass`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `lower_mass`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
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
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `top_k_row_span_sum`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 81,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
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
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::connected_component`,
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `top_k_row_right_mass_sum`,
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
                                generic_parameters: DeclarativeGenericParameters {
                                    data: [],
                                },
                                ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::connected_component::ConnectedComponent`),
                            generic_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 81,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
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