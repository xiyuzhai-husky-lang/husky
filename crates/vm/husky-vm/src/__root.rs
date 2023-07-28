use crate::*;
use husky_dev_utils::*;

pub static __EQ_LINKAGE: __LinkageGroup = transfer_linkage!(
    |values, _| unsafe { (values[0] == values[1]).__to_register() },
    none
);

pub static __NEQ_LINKAGE: __LinkageGroup = transfer_linkage!(
    |values, _| unsafe { (values[0] != values[1]).__to_register() },
    none
);

pub static __ASSIGN_LINKAGE: __LinkageGroup = transfer_linkage!(
    |args, _| {
        assert_eq!(args[0].vtable as *const _, args[1].vtable as *const _);
        unsafe { (args[0].vtable.assign)(args.as_mut_ptr()) }.to_register()
    },
    none
);

pub static __VALUE_CALL_LINKAGE: __LinkageGroup = transfer_linkage!(
    |values, opt_ctx| {
        let call_form_value: &__VirtualFunction =
            values[0].downcast_temp_ref(&__VIRTUAL_FUNCTION_VTABLE);
        match call_form_value {
            __VirtualFunction::ThickFp(resolved_linkage) => {
                resolved_linkage.call(opt_ctx, &mut values[1..])
            }
        }
    },
    none
);
