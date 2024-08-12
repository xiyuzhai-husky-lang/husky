```rust
[
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
                    context: LinTypeContext {
                        comptime_var_overrides: [],
                    },
                    variable_resolutions: [],
                    separator: None,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A`),
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
                                value: 16,
                            },
                        ),
                    ),
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
                    index: 1,
                },
            },
        },
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::UnitVariant`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithOneField`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::PropsVariantWithTwoFields`),
                instantiation: LinInstantiation {
                    path: ItemPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithTwoFields`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantConstructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDiscriminator {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantDestructor {
                self_ty: LinTypePathLeading {
                    ty_path: TypePath(`syntax_basics::defn::major_item::ty::enum_ty::A`, `Enum`),
                    template_arguments: [],
                },
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
        Linket {
            data: LinketData::EnumVariantField {
                path: TypeVariantPath(`syntax_basics::defn::major_item::ty::enum_ty::A::TupleVariantWithOneField`),
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
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
        None,
    ),
    (
        Linket {
            data: LinketData::MajorFunctionRitchie {
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
        Some(
            VmirRegion {
                linket: Linket {
                    data: LinketData::MajorFunctionRitchie {
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
                root_expr: VmirExprIdx(
                    2,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Literal {
                            value: I32(
                                1,
                            ),
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    1..2,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Eval {
                            expr: VmirExprIdx(
                                0,
                            ),
                            coercion: None,
                            discarded: false,
                        },
                        VmirStmtData::Let {
                            pattern: VmirPattern {
                                restructive_pattern: VmirRestructivePattern::Default(
                                    None,
                                ),
                                destructive_pattern: None,
                            },
                            initial_value: VmirExprIdx(
                                1,
                            ),
                            coercion: None,
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linket {
            data: LinketData::MajorFunctionRitchie {
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
        Some(
            VmirRegion {
                linket: Linket {
                    data: LinketData::MajorFunctionRitchie {
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
                root_expr: VmirExprIdx(
                    1,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Closure,
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Let {
                            pattern: VmirPattern {
                                restructive_pattern: VmirRestructivePattern::Default(
                                    None,
                                ),
                                destructive_pattern: None,
                            },
                            initial_value: VmirExprIdx(
                                0,
                            ),
                            coercion: None,
                        },
                    ],
                },
            },
        ),
    ),
    (
        Linket {
            data: LinketData::MajorFunctionRitchie {
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
        Some(
            VmirRegion {
                linket: Linket {
                    data: LinketData::MajorFunctionRitchie {
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
                root_expr: VmirExprIdx(
                    1,
                ),
                vmir_expr_arena: Arena {
                    data: [
                        VmirExprData::Closure,
                        VmirExprData::Block {
                            stmts: VmirStmtIdxRange(
                                ArenaIdxRange(
                                    0..1,
                                ),
                            ),
                            destroyers: ArenaIdxRange(
                                0..0,
                            ),
                        },
                    ],
                },
                vmir_stmt_arena: Arena {
                    data: [
                        VmirStmtData::Let {
                            pattern: VmirPattern {
                                restructive_pattern: VmirRestructivePattern::Default(
                                    None,
                                ),
                                destructive_pattern: None,
                            },
                            initial_value: VmirExprIdx(
                                0,
                            ),
                            coercion: None,
                        },
                    ],
                },
            },
        ),
    ),
]
```