#[macro_export]
macro_rules! memo_linket_impl {
    ($memo: path, $item_path_id_interface: path) => {
        __LinketImpl::Memo {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            ki_wrapper: |__self| {
                __KiControlFlow::Continue(
                    $memo(<_ as __FromValue>::from_value_static(__self)).into_value(),
                )
            },
        }
    };
}
