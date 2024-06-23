```rust
[
    (
        ItemPath(`core::num::i8`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::i8`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::i16`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::i32`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::i64`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::i128`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::isize`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::u8`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::u16`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::u32`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::u64`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::u128`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::usize`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::f32`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::num::f64`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::i8(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`i8`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8(0)::max`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i8 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::i8 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add i8`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 52,
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
        ItemPath(`<core::num::i8 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 2,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::i16(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`i16`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16(0)::max`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i16 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::i16 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add i32`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 53,
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
        ItemPath(`<core::num::i16 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::i32(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`i32`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::max`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32(0)::min`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i32 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::i32 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add i32`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 54,
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
        ItemPath(`<core::num::i32 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::i64(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`i64`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i64 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::i64 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add i64`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 55,
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
        ItemPath(`<core::num::i64 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 5,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::i128(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`i128`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::i128 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::i128 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add i128`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 56,
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
        ItemPath(`<core::num::i128 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::isize(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`isize`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::isize as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::isize as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add isize`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 57,
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
        ItemPath(`<core::num::isize as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::u8(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`u8`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u8 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::u8 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add u8`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 58,
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
        ItemPath(`<core::num::u8 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 8,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::u16(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`u16`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u16 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::u16 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add u16`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 59,
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
        ItemPath(`<core::num::u16 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 9,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::u32(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`u32`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u32 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::u32 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add u32`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 60,
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
        ItemPath(`<core::num::u32 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 10,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::u64(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`u64`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u64 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::u64 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add u64`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 61,
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
        ItemPath(`<core::num::u64 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 11,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::u128(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`u128`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 15,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::u128 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::u128 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add u128`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 62,
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
        ItemPath(`<core::num::u128 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 12,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::usize(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`usize`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 16,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::usize as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::usize as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add usize`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 63,
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
        ItemPath(`<core::num::usize as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::f32(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`f32`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 17,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sqrt`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::max`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::min`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sgnx`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::cos`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::sin`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32(0)::acos`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f32 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::f32 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add f32`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 64,
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
        ItemPath(`<core::num::f32 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 14,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::num::f64(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`f64`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)::abs`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64(0)::acos`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::num::f64 as core::ops::Add(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::num::f64 as core::ops::Add(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        trai: EthTerm(`Add f64`),
                        self_ty_refined: PathLeading(
                            ItemPath(
                                TypeOntology(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 65,
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
        ItemPath(`<core::num::f64 as core::ops::Add(0)>::add`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    MethodRitchie(
                        TraitForTypeMethodRitchieEthTemplate(
                            Id {
                                value: 15,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```