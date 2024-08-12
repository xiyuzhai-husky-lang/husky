#[macro_export]
macro_rules! val_linket_impl {
    ($val: path, $item_path_id_interface: path) => {
        __LinketImpl::Val {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            ki_wrapper: || __ki_catch_unwind!($val).map(__IntoValue::into_value),
        }
    };
}

#[test]
fn val_linket_impl_works() {
    use crate::ugly::*;
    use husky_item_path_interface::ItemPathIdInterface;
    use husky_linket_impl::__ki_catch_unwind;
    use husky_linket_impl::exception::TrackedException;
    use husky_linket_impl::ki_catch_unwind;

    fn val() {}

    static mut VAL_INTERFACE: Option<ItemPathIdInterface> = None;

    val_linket_impl!(val, VAL_INTERFACE);
}
