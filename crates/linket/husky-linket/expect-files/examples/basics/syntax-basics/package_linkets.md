```rust
[
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 13,
                        },
                    ),
                ),
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 13,
                        },
                    ),
                ),
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Props {
                ident: Ident(
                    Coword(
                        Id {
                            value: 16,
                        },
                    ),
                ),
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantConstructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDiscriminator {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantDestructor {
            self_ty: LinTypePathLeading {
                ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                template_arguments: [],
            },
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 0,
            },
        },
    },
    Linket {
        data: LinketData::EnumVariantField {
            path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
            field_ty_leash_class: Copyable,
            field: Tuple {
                index: 1,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`syntax_basics::expr::nested`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::expr::nested`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`syntax_basics::expr::closure_inline`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::expr::closure_inline`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
    Linket {
        data: LinketData::MajorRitchie {
            path: MajorFormPath(`syntax_basics::expr::closure_nested`, `Ritchie(
                Fn,
            )`),
            instantiation: LinInstantiation {
                path: ItemPath(`syntax_basics::expr::closure_nested`),
                context: LinTypeContext {
                    comptime_var_overrides: [],
                },
                variable_resolutions: [],
                separator: None,
            },
        },
    },
]
```