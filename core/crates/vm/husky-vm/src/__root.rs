use crate::*;
use dev_utils::*;

pub static __EQ_LINKAGE: __Linkage = specific_transfer_linkage!(|_, __values| {
    __TempValue::Copyable((__values[0].any_ref().__equal_any(__values[1].any_ref())).into())
});

pub static __NEQ_LINKAGE: __Linkage = specific_transfer_linkage!(|_, __values| {
    __TempValue::Copyable((!__values[0].any_ref().__equal_any(__values[1].any_ref())).into())
});
