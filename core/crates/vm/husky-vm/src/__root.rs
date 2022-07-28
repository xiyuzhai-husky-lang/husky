use crate::*;
use husky_dev_utils::*;

pub static __EQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, __values| unsafe { (__values[0] == __values[1]).__to_register__() },
    none
);

pub static __NEQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, __values| unsafe { (__values[0] != __values[1]).__to_register__() },
    none
);

pub static __VALUE_CALL_LINKAGE: __Linkage = transfer_linkage!(
    |ctx, values| unsafe {
        let call_form_value: &__VirtualFunction = values[0].downcast_temp_ref();
        match call_form_value {
            __VirtualFunction::Fp(linkage_fp) => linkage_fp.call(ctx, &mut values[1..]),
        }
    },
    none
);
