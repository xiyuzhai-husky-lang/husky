[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
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
                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativePoint2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
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
                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `y`,
                                    ty: DeclarativeTerm(`core::num::f32`),
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
                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `min`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `max`,
                                    ty: DeclarativeTerm(`core::num::f32`),
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
                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `yrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
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
                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            fields: [
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                },
                                PropsStructFieldDeclarativeSignatureTemplate {
                                    ident: `yrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
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
                        value: 33,
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
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `vector`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 83,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `norm`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 83,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
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
                        value: 34,
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
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `point`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
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
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::Point2d`),
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `norm`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::Vector2d`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
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
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
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
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 36,
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
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `xmin`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `xmax`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `ymin`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `ymax`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::BoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 87,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
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
                        value: 37,
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
                        ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `xmin`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `xmax`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `ymin`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `ymax`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativeBoundingBox`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitRegularParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 88,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]