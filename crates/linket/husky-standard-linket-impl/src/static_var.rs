use crate::*;
use husky_linket_impl::var_id::IsVarId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StandardVarId {
    data: [u64; 4],
}

impl IsVarId for StandardVarId {}

impl StandardVarId {
    pub fn new(data: [u64; 4]) -> Self {
        Self { data }
    }
}

impl From<u32> for StandardVarId {
    fn from(data: u32) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<u32> for StandardVarId {
    fn into(self) -> u32 {
        self.data[0] as u32
    }
}

impl From<u64> for StandardVarId {
    fn from(data: u64) -> Self {
        Self {
            data: [data, 0, 0, 0],
        }
    }
}

impl Into<u64> for StandardVarId {
    fn into(self) -> u64 {
        self.data[0]
    }
}

impl From<usize> for StandardVarId {
    fn from(data: usize) -> Self {
        Self {
            data: [data as u64, 0, 0, 0],
        }
    }
}

impl Into<usize> for StandardVarId {
    fn into(self) -> usize {
        self.data[0] as usize
    }
}

impl From<[u64; 4]> for StandardVarId {
    fn from(data: [u64; 4]) -> Self {
        Self { data }
    }
}

#[macro_export]
macro_rules! static_var_linket_impl {
    ($var_path: path, $item_path_id_interface: path) => {
        __LinketImpl::StaticVar {
            init_item_path_id_interface: |item_path_id_interface| unsafe {
                $item_path_id_interface = Some(item_path_id_interface)
            },
            get_var_id: <$var_path as __IsStaticVar<__VarId>>::get_id,
            try_set_var_id: |id, locked| unsafe {
                <$var_path as __IsStaticVar<__VarId>>::try_set_var_id(id, locked)
                    .map(|restore| -> Box<dyn FnOnce()> { Box::new(restore) })
            },
            page_var_ids: |locked, page_start, page_limit| {
                Box::new(<$var_path as __IsStaticVar<__VarId>>::page_var_ids(
                    locked, page_start, page_limit,
                ))
            },
            default_page_start: <$var_path as __IsStaticVar<__VarId>>::default_page_start,
            get_value: <$var_path as __IsStaticVar<__VarId>>::get_value,
        }
    };
}

#[test]
fn static_var_linket_impl_works() {
    use crate::{pedestal::StandardPedestal, static_var::StandardVarId, ugly::*};
    use husky_linket_impl::static_var::IsStaticVar;
    use StandardLinketImpl as __LinketImpl;

    #[allow(non_camel_case_types)]
    struct STATIC_VAR_A {}

    impl __IsStaticVar<__VarId> for STATIC_VAR_A {
        fn item_path_id_interface() -> ItemPathIdInterface {
            todo!()
        }

        fn page_var_ids_aux(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = __VarId> {
            (0..10u32).map(Into::into)
        }

        fn get_id() -> __VarId {
            todo!()
        }

        fn try_set_var_id_aux(
            id: __VarId,
            locked: &[ItemPathIdInterface],
        ) -> __ThawedVarResult<impl FnOnce() + 'static> {
            todo!();
            Ok(|| todo!())
        }

        type Value = Value;

        fn get_value() -> Self::Value {
            todo!()
        }

        fn default_page_start(locked: &[ItemPathIdInterface]) -> StaticVarResult<__VarId, __VarId> {
            todo!()
        }
    }

    /// We use the same name
    thread_local! {
        pub static STATIC_VAR_A: std::cell::Cell<i32> = Default::default();
    }

    #[allow(non_upper_case_globals)]
    pub static mut STATIC_VAR_A__ITEM_PATH_ID_INTERFACE: Option<ItemPathIdInterface> = None;

    let LinketImpl::StaticVar {
        init_item_path_id_interface,
        get_var_id: get_id,
        try_set_var_id: try_replace_id,
        page_var_ids: ids,
        ..
    } = static_var_linket_impl!(STATIC_VAR_A, STATIC_VAR_A__ITEM_PATH_ID_INTERFACE)
    else {
        unreachable!()
    };
}
