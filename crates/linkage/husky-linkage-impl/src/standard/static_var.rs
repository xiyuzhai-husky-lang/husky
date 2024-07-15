use super::*;

#[macro_export]
macro_rules! static_var_linkage_impl {
    ($static_var: path) => {
        __LinkageImpl::StaticVar {
            set_up_for_testing: <$static_var>::set_up_for_testing,
        }
    };
}

#[test]
fn static_var_linkage_impl_works() {
    use StandardLinkageImpl as __LinkageImpl;

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A {}

    impl STATIC_VAR_A {
        fn set_up_for_testing(index: usize) {
            STATIC_VAR_A.set(index.try_into().unwrap())
        }
    }

    /// We use the same name
    thread_local! {
        static STATIC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    let LinkageImpl::<()>::StaticVar { set_up_for_testing } =
        static_var_linkage_impl!(STATIC_VAR_A)
    else {
        unreachable!()
    };
    set_up_for_testing(45);
    assert_eq!(STATIC_VAR_A.get(), 45);
}
