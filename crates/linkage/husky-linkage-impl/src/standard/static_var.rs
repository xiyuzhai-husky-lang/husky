use super::*;

#[macro_export]
macro_rules! static_var_linkage_impl {
    ($p: path) => {
        __LinkageImpl::StaticVar {
            set_up_for_testing: todo!(),
        }
    };
}

fn static_var_linkage_impl_works() {
    use LinkageImpl as __LinkageImpl;

    thread_local! {
        static STATC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A;

    let _: LinkageImpl<()> = static_var_linkage_impl!(STATIC_VAR_A);
    todo!()
}
