use super::*;

#[macro_export]
macro_rules! static_var_linket_impl {
    ($static_var: path) => {
        __LinketImpl::StaticVar {
            set_up_for_testing: <$static_var>::set_up_for_testing,
            get_id: <$static_var>::get_id,
            set_id: <$static_var>::set_id,
        }
    };
}

#[test]
fn static_var_linket_impl_works() {
    use StandardLinketImpl as __LinketImpl;

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A {}

    impl STATIC_VAR_A {
        pub fn set_up_for_testing(index: usize) {
            STATIC_VAR_A.set(index.try_into().unwrap())
        }

        pub fn get_id() -> () {
            ()
        }

        pub fn set_id(id: ()) {
            ()
        }
    }

    /// We use the same name
    thread_local! {
        static STATIC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    let LinketImpl::<()>::StaticVar {
        set_up_for_testing,
        get_id,
        set_id,
    } = static_var_linket_impl!(STATIC_VAR_A)
    else {
        unreachable!()
    };
    set_up_for_testing(45);
    assert_eq!(STATIC_VAR_A.get(), 45);
}
