use super::*;

#[macro_export]
macro_rules! static_var_linket_impl {
    ($static_var: path, $item_path_id_interface: path) => {
        __LinketImpl::StaticVar {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            set_up_for_testing: <$static_var>::set_up_for_testing,
            get_id: <$static_var>::get_id,
            set_id: <$static_var>::set_id,
        }
    };
}

#[test]
fn static_var_linket_impl_works() {
    use husky_standard_devsoul_interface::pedestal::StandardPedestal;
    use husky_standard_devsoul_interface::static_var::StandardStaticVarId;
    use StandardLinketImpl as __LinketImpl;

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A {}

    impl STATIC_VAR_A {
        pub fn set_up_for_testing(index: usize) {
            STATIC_VAR_A.set(index.try_into().unwrap())
        }

        pub fn get_id() -> StandardStaticVarId {
            todo!()
            // StandardStaticVarId::
        }

        pub fn set_id(id: StandardStaticVarId) {
            todo!()
        }
    }

    /// We use the same name
    thread_local! {
        pub static STATIC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    #[allow(non_upper_case_globals)]
    pub static mut STATIC_VAR_A__ITEM_PATH_ID_INTERFACE: Option<ItemPathIdInterface> = None;

    let LinketImpl::<StandardPedestal>::StaticVar {
        init_item_path_id_interface,
        set_up_for_testing,
        get_id,
        set_id,
    } = static_var_linket_impl!(STATIC_VAR_A, STATIC_VAR_A__ITEM_PATH_ID_INTERFACE)
    else {
        unreachable!()
    };
    set_up_for_testing(45);
    assert_eq!(STATIC_VAR_A.get(), 45);
}
