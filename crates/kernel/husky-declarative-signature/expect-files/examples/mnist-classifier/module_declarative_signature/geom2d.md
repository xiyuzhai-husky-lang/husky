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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `x`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `min`,
                                    ty: DeclarativeTerm(`core::num::f32`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    TypeDeclarativeSignatureTemplate::RegularStruct(
                        RegularStructDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            fields: [
                                RegularStructFieldDeclarativeSignatureTemplate {
                                    ident: `xrange`,
                                    ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                                },
                                RegularStructFieldDeclarativeSignatureTemplate {
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
                    ident: `from_i_shift28`,
                    item_kind: AssociatedFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::AssociatedFn(
                        TypeAssociatedFnDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 82,
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `to`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 82,
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
                                                        value: 82,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 82,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `dist`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 82,
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
                                                        value: 82,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                    ident: `to`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 84,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `dot`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 84,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `cross`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 84,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `angle`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 36,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `rotation_direction_to`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 84,
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `angle_to`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 84,
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
                                                        value: 84,
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
                                                        value: 36,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `relative_range`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignatureTemplate {
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
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
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
                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `relative_point`,
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
                                ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            },
                            self_ty: DeclarativeTerm(`mnist_classifier::geom2d::ClosedRange`),
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
                                data: [
                                    ExplicitParameterDeclarativeSignatureTemplate {
                                        contract: Pure,
                                        ty: EntityPath(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 60,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
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
                    ident: `relative_bounding_box`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
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
                                                        value: 86,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
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
                        module_path: `mnist_classifier::geom2d`,
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                        disambiguator: 0,
                    },
                    ident: `relative_point`,
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
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
                                                        value: 82,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            return_ty: DeclarativeTerm(`mnist_classifier::geom2d::RelativePoint2d`),
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
                                contract: Pure,
                                ty: EntityPath(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 86,
                                            },
                                        ),
                                    ),
                                ),
                            },
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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
                            self_parameter: ExplicitParameterDeclarativeSignatureTemplate {
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
                            nonself_regular_parameters: ExplicitParameterDeclarativeSignatureTemplates {
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