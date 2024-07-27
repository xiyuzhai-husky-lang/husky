#[macro_export]
macro_rules! val_linket_impl {
    ($val: path, $item_path_id_interface: path) => {
        __LinketImpl::Val {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            ki_wrapper: || __KiControlFlow::Continue($val().into_value()),
        }
    };
}
