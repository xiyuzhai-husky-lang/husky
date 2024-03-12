use super::*;

#[macro_export]
macro_rules! enum_variant_field_linkage_impl {
    ($self_ty: ty, $variant_path: path, $field: ident) => {{
        fn struct_field_wrapper(owner: Value) -> Value {
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
        LinkageImpl::StructField {
            struct_field_wrapper,
        }
    }};
}

#[test]
fn enum_variant_field_linkage_impl_works() {
    use crate::standard::ugly::__ValueLeashTest;
    enum Animal {
        Cat { weight: i32 },
        Dog { weight: i32 },
    }

    let _: LinkageImpl<()> = enum_variant_field_linkage_impl!(Animal, Animal::Cat, weight);
}
