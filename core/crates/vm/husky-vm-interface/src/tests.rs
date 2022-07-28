use crate::*;

#[test]
fn downcast_works_for_bool() {
    let mut ra = {
        let a = true;
        unsafe { a.__to_register__() }
    };
    let b: bool = ra.downcast_bool();
    assert!(b)
}

#[test]
fn downcast_works_for_i32() {
    let mut ra = {
        let a = 1i32;
        unsafe { a.__to_register__() }
    };
    let b: i32 = ra.downcast_i32();
}

#[test]
fn it_works() {
    unsafe {
        assert!(
            __Register::new_primitive_value(__RegisterData { as_bool: true }, &__BOOL_VTABLE)
                .downcast_bool()
        );
        assert!(__bool_primitive_value_to_bool(__RegisterData {
            as_bool: true
        }))
    }
}
