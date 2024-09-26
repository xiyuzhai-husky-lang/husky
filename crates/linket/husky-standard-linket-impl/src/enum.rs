#[cfg(test)]
use super::*;

/// # constructor

#[macro_export]
macro_rules! enum_variant_constructor_linket_impl {
    ($self_ty: ty, $variant_path: path) => {{
        fn enum_variant_constructor_ki_wrapper(args: &[__KiArgumentReprInterface]) -> __Value {
            todo!()
        }
        fn enum_variant_constructor_vm_wrapper(_args: __VmArgumentValues) -> __ThawedValue {
            let r: $self_ty = $variant_path;
            r.into_thawed_value()
        }
        __LinketImpl::EnumVariantConstructor {
            enum_variant_constructor_ki_wrapper,
            enum_variant_constructor_vm_wrapper,
        }
    }}; // todo: props and tuple variant
    ($self_ty: ty, $variant_path: path, {$($fields: ident),* $(,)?}) => {{
        fn enum_variant_constructor_ki_wrapper(args: &[__KiArgumentReprInterface]) -> __Value {
            todo!()
        }
        fn enum_variant_constructor_vm_wrapper(args: __VmArgumentValues) -> __ThawedValue {
            todo!()
        }
        __LinketImpl::EnumVariantConstructor {
            enum_variant_constructor_ki_wrapper,
            enum_variant_constructor_vm_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ($($fields: ident),* $(,)?)) => {{
        fn enum_variant_constructor_ki_wrapper(args: &[__KiArgumentReprInterface]) -> __Value {
            todo!()
        }
        fn enum_variant_constructor_vm_wrapper(_args: __VmArgumentValues) -> __ThawedValue {
            todo!()
        }
        __LinketImpl::EnumVariantConstructor {
            enum_variant_constructor_ki_wrapper,
            enum_variant_constructor_vm_wrapper,
        }
    }};
}

/// # destructor

#[macro_export]
macro_rules! enum_variant_destructor_linket_impl {
    ($self_ty: ty, $variant_path: path, {$($fields: ident),* $(,)?}) => {{
        fn enum_variant_destructor_wrapper(owner: Value) -> Vec<Value> {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path { $($fields),* } = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    vec![$($fields.into_value()),*]
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantDestructor {
            enum_variant_destructor_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ($($fields: ident),* $(,)?)) => {{
        fn enum_variant_destructor_wrapper(owner: Value) -> Vec<Value> {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path($($fields),*) = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    vec![$($fields.into_value()),*]
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantDestructor {
            enum_variant_destructor_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_destructor_linket_impl_works() {
    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: StandardLinketImpl =
        enum_variant_destructor_linket_impl!(Animal, Animal::Cat, { height, weight });

    let _: StandardLinketImpl =
        enum_variant_destructor_linket_impl!(Animal, Animal::Dog, { weight });
}

#[test]
fn enum_tuple_variant_destructor_linket_impl_works() {
    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: StandardLinketImpl =
        enum_variant_destructor_linket_impl!(Animal, Animal::Cat, (height, weight));

    let _: StandardLinketImpl = enum_variant_destructor_linket_impl!(Animal, Animal::Dog, (weight));
}

/// # discriminator

#[macro_export]
macro_rules! enum_variant_discriminator_linket_impl {
    ($self_ty: ty, $variant_path: path) => {{
        fn enum_variant_discriminator_wrapper(owner: Value) -> bool {
            match owner {
                Value::Owned(owner) => {
                    matches!(owner.downcast_into_owned::<$self_ty>(), $variant_path)
                }
                Value::Leash(owner) => {
                    matches!(
                        (owner as &'static dyn std::any::Any)
                            .downcast_ref::<$self_ty>()
                            .unwrap(),
                        $variant_path
                    )
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantDiscriminator {
            enum_variant_discriminator_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, {}) => {{
        fn enum_variant_discriminator_wrapper(owner: Value) -> bool {
            match owner {
                Value::Owned(owner) => {
                    matches!(
                        owner.downcast_into_owned::<$self_ty>(),
                        $variant_path { .. }
                    )
                }
                Value::Leash(owner) => {
                    matches!(
                        (owner as &'static dyn std::any::Any)
                            .downcast_ref::<$self_ty>()
                            .unwrap(),
                        $variant_path { .. }
                    )
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantDiscriminator {
            enum_variant_discriminator_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ()) => {{
        fn enum_variant_discriminator_wrapper(owner: Value) -> bool {
            match owner {
                Value::Owned(owner) => {
                    matches!(owner.downcast_into_owned::<$self_ty>(), $variant_path(..))
                }
                Value::Leash(owner) => {
                    matches!(
                        (owner as &'static dyn std::any::Any)
                            .downcast_ref::<$self_ty>()
                            .unwrap(),
                        $variant_path(..)
                    )
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantDiscriminator {
            enum_variant_discriminator_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_discriminator_linket_impl_works() {
    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Frog, {});
    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Dog, {});
    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Cat, {});
}

#[test]
fn enum_tuple_variant_discriminator_linket_impl_works() {
    use crate::pedestal::StandardPedestal;

    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Frog, ());
    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Dog, ());
    let _: StandardLinketImpl = enum_variant_discriminator_linket_impl!(Animal, Animal::Cat, ());
}

/// # field

#[macro_export]
macro_rules! enum_variant_field_linket_impl {
    ($self_ty: ty, $variant_path: path, {$class: ident $field: ident}) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path { $field, .. } = owner.downcast_into_owned::<$self_ty>()
                    else {
                        unreachable!()
                    };
                    $field.into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path { $field, .. } = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    class_specific_leashed_field_into_value!($class $field)
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ($class: ident v0)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(v0, ..) = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    v0.into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(v0, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    class_specific_leashed_field_into_value!($class v0)
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ($class: ident v1)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(_, v1, ..) = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    v1.into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(_, v1, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    class_specific_leashed_field_into_value!($class v1)
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, ($class: ident v2)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(_, _, v2, ..) = owner.downcast_into_owned::<$self_ty>()
                    else {
                        unreachable!()
                    };
                    v1.into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(_, _, v2, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    class_specific_leashed_field_into_value!($class v2)
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_field_linket_impl_works() {
    use husky_core::*;

    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: StandardLinketImpl =
        enum_variant_field_linket_impl!(Animal, Animal::Cat, { copyable weight });
}

#[test]
fn enum_tuple_variant_field_linket_impl_works() {
    use crate::pedestal::StandardPedestal;
    use husky_core::*;

    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: StandardLinketImpl = enum_variant_field_linket_impl!(Animal, Animal::Dog, (copyable v0));
    let _: StandardLinketImpl = enum_variant_field_linket_impl!(Animal, Animal::Cat, (copyable v0));
    let _: StandardLinketImpl = enum_variant_field_linket_impl!(Animal, Animal::Cat, (copyable v1));
}

#[macro_export]
macro_rules! enum_index_presenter_linket_impl {
    ($ty: ty) => {
        __LinketImpl::EnumUnitValuePresenter {
            presenter: |index: usize, _, _| {
                let index: u8 = index.try_into().unwrap();
                let slf: $ty = unsafe { std::mem::transmute(index) };
                __ValuePresentation::AdHoc(format!("{slf:?}"))
            },
        }
    };
}

#[test]
#[ignore]
fn enum_index_presenter_linket_impl_works() {
    todo!()
}
