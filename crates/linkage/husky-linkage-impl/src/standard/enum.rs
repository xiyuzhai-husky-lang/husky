#[cfg(test)]
use super::*;

/// # constructor

macro_rules! enum_variant_constructor_linkage_impl {
    ($self_ty: ty, $variant_path: path) => {{
        // ad hoc
        fn enum_variant_constructor_wrapper(owner: Value) -> Vec<Value> {
            todo!()
        }
        LinkageImpl::EnumVariantConstructor {
            enum_variant_constructor_wrapper,
        }
    }};
}

/// # destructor

#[macro_export]
macro_rules! enum_variant_destructor_linkage_impl {
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
        LinkageImpl::EnumVariantDestructor {
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
        LinkageImpl::EnumVariantDestructor {
            enum_variant_destructor_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_destructor_linkage_impl_works() {
    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: LinkageImpl<()> =
        enum_variant_destructor_linkage_impl!(Animal, Animal::Cat, { height, weight });

    let _: LinkageImpl<()> = enum_variant_destructor_linkage_impl!(Animal, Animal::Dog, { weight });
}

#[test]
fn enum_tuple_variant_destructor_linkage_impl_works() {
    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: LinkageImpl<()> =
        enum_variant_destructor_linkage_impl!(Animal, Animal::Cat, (height, weight));

    let _: LinkageImpl<()> = enum_variant_destructor_linkage_impl!(Animal, Animal::Dog, (weight));
}

/// # discriminator

#[macro_export]
macro_rules! enum_variant_discriminator_linkage_impl {
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
                Value::Ref(owner) => todo!("enum_props_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_props_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantDiscriminator {
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
                Value::Ref(owner) => todo!("enum_tuple_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_tuple_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantDiscriminator {
            enum_variant_discriminator_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_discriminator_linkage_impl_works() {
    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Frog, {});
    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Dog, {});
    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Cat, {});
}

#[test]
fn enum_tuple_variant_discriminator_linkage_impl_works() {
    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Frog, ());
    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Dog, ());
    let _: LinkageImpl<()> = enum_variant_discriminator_linkage_impl!(Animal, Animal::Cat, ());
}

/// # field

#[macro_export]
macro_rules! enum_variant_field_linkage_impl {
    ($self_ty: ty, $variant_path: path, {$field: ident}) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path { $field, .. } = owner.downcast_into_owned::<$self_ty>()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest($field).into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path { $field, .. } = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest(($field as &'static _)).into_value()
                }
                Value::Ref(owner) => todo!("enum_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, (v0)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(v0, ..) = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    __ValueLeashTest(v0).into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(v0, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest((v0 as &'static _)).into_value()
                }
                Value::Ref(owner) => todo!("enum_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, (v1)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(_, v1, ..) = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    __ValueLeashTest(v1).into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(_, v1, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest((v1 as &'static _)).into_value()
                }
                Value::Ref(owner) => todo!("enum_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
    ($self_ty: ty, $variant_path: path, (v2)) => {{
        fn enum_variant_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    let $variant_path(_, _, v2, ..) = owner.downcast_into_owned::<$self_ty>()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest(v1).into_value()
                }
                Value::Leash(owner) => {
                    let $variant_path(_, _, v2, ..) = (owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                    else {
                        unreachable!()
                    };
                    __ValueLeashTest((v2 as &'static _)).into_value()
                }
                Value::Ref(owner) => todo!("enum_variant_field_wrapper Ref"),
                Value::Mut(owner) => todo!("enum_variant_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::EnumVariantField {
            enum_variant_field_wrapper,
        }
    }};
}

#[test]
fn enum_props_variant_field_linkage_impl_works() {
    use crate::standard::ugly::__ValueLeashTest;

    enum Animal {
        Frog {},
        Dog { weight: i32 },
        Cat { height: i32, weight: i32 },
    }

    let _: LinkageImpl<()> = enum_variant_field_linkage_impl!(Animal, Animal::Cat, { weight });
}

#[test]
fn enum_tuple_variant_field_linkage_impl_works() {
    use crate::standard::ugly::__ValueLeashTest;

    enum Animal {
        Frog(),
        Dog(i32),
        Cat(i32, i32),
    }

    let _: LinkageImpl<()> = enum_variant_field_linkage_impl!(Animal, Animal::Dog, (v0));
    let _: LinkageImpl<()> = enum_variant_field_linkage_impl!(Animal, Animal::Cat, (v0));
    let _: LinkageImpl<()> = enum_variant_field_linkage_impl!(Animal, Animal::Cat, (v1));
}
