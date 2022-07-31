use crate::*;
use husky_dev_utils::*;

pub static __EQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, values| unsafe { (values[0] == values[1]).__to_register__() },
    none
);

pub static __NEQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, values| unsafe { (values[0] != values[1]).__to_register__() },
    none
);

pub static __ASSIGN_LINKAGE: __Linkage = transfer_linkage!(
    |_, args| {
        assert_eq!(args[0].vtable as *const _, args[1].vtable as *const _);
        args[0].vtable.assign.unwrap()(args.as_mut_ptr()).to_register()
    },
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
