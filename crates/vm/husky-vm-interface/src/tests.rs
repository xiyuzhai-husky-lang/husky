use crate::*;

#[test]
fn downcast_works_for_bool() {
    let mut ra = {
        let a = true;
        unsafe { a.__to_register() }
    };
    let b: bool = ra.downcast_bool();
    assert!(b)
}

#[test]
fn downcast_works_for_i32() {
    let mut ra = {
        let a = 1i32;
        unsafe { a.__to_register() }
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

#[test]
fn it() {
    unsafe { assert_eq!(__RegisterData { as_i32: 1 }.as_b64, 1) }
}

#[test]
fn test_register_data_size() {
    assert_eq!(std::mem::size_of::<f64>(), std::mem::size_of::<*mut ()>(),);
    assert_eq!(
        std::mem::size_of::<f64>(),
        std::mem::size_of::<__RegisterData>()
    )
}

// C standard (N1570, 6.7.2.1 Structure and union specifiers) says:
// 16 The size of a union is sufficient to contain the largest of its members.
// The value of at most one of the members can be stored in a union object at any time.
// A pointer to a union object, suitably converted, points to each of its members
// (or if a member is a bit- field, then to the unit in which it resides),
// and vice versa.
#[test]
fn test_alignment() {
    let a = __RegisterData { as_void: () };
    unsafe {
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_void as *const _ as *const (),
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_bool as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_i64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_b64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f32 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_f64 as *const _ as *const ()
        );
        assert_eq!(
            &a as *const _ as *const (),
            &a.as_ptr as *const _ as *const ()
        )
    }
}
