#[cfg(test)]
use super::*;
#[cfg(test)]
use husky_linket_impl::standard::ugly::serde;

/// # destructor

#[macro_export]
macro_rules! struct_destructor_linket_impl {
    ($self_ty: ty, $ty_path: path, $($fields: ident),* $(,)?) => {{
        fn struct_destructor_wrapper(owner: Value) -> Vec<Value> {
            match owner {
                Value::Owned(owner) => {
                    let $ty_path { $($fields),* } = owner.downcast_into_owned::<$self_ty>() else {
                        unreachable!()
                    };
                    vec![$($fields.into_value()),*]
                }
                _ => unreachable!(),
            }
        }
        __LinketImpl::StructDestructor {
            struct_destructor_wrapper,
        }
    }};
}

// # field

#[macro_export]
macro_rules! struct_field_linket_impl {
    ($self_ty: ty, $class: ident $field: ident) => {{
        fn struct_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => owner.downcast_into_owned::<$self_ty>().$field.into_value(),
                Value::Leash(owner) => class_specific_leashed_field_into_value!(
                    $class
                        & (owner as &'static dyn std::any::Any)
                            .downcast_ref::<$self_ty>()
                            .unwrap()
                            .$field
                ),
                Value::Ref(owner) => todo!("struct_field_wrapper Ref"),
                Value::Mut(owner) => todo!("struct_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        __LinketImpl::StructField {
            struct_field_wrapper,
        }
    }};
}

#[test]
fn struct_field_linket_impl_works() {
    use husky_core::*;
    use husky_standard_value::ugly::*;

    #[husky_standard_value::value_conversion]
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct B(Vec<i32>);

    struct A {
        x: i32,
        y: Vec<i32>,
        b: B,
    }

    let _: StandardLinketImpl<()> = struct_field_linket_impl!(A, copyable x);
    let _: StandardLinketImpl<()> = struct_field_linket_impl!(A, vec y);
    let _: StandardLinketImpl<()> = struct_field_linket_impl!(A, other b);
}
