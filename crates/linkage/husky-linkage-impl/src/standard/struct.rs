use super::*;

#[macro_export]
macro_rules! struct_field_linkage_impl {
    ($self_ty: ty, $field: ident) => {{
        fn struct_field_wrapper(owner: Value) -> Value {
            match owner {
                Value::Owned(owner) => {
                    __ValueLeashTest(owner.downcast_into_owned::<$self_ty>().$field).into_value()
                }
                Value::Leash(owner) => __ValueLeashTest(
                    (&((owner as &'static dyn std::any::Any)
                        .downcast_ref::<$self_ty>()
                        .unwrap()
                        .$field) as &'static _),
                )
                .into_value(),
                Value::Ref(owner) => todo!("struct_field_wrapper Ref"),
                Value::Mut(owner) => todo!("struct_field_wrapper Mut"),
                _ => unreachable!(),
            }
        }
        LinkageImpl::StructField {
            struct_field_wrapper,
        }
    }};
}

#[test]
fn struct_field_linkage_impl_works() {
    use crate::standard::ugly::__ValueLeashTest;
    struct A {
        x: i32,
    }

    let _: LinkageImpl<()> = struct_field_linkage_impl!(A, x);
}
