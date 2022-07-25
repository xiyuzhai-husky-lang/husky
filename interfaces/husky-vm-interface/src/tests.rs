use crate::*;

#[derive(Debug)]
struct A {}

impl __StaticInfo for A {
    type __StaticSelf = A;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "A".into()
    }
}

impl __Registrable for A {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }
}

#[test]
fn downcast_works() {
    let a = A {};
    let mut ra = __Register::new_box(a);
    let b: A = ra.downcast();
}
