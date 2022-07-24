use crate::*;
use husky_dev_utils::*;

pub static __EQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, __values| unsafe { (__values[0].__eq__(&__values[1])).__to_register__() },
    none
);

pub static __NEQ_LINKAGE: __Linkage = transfer_linkage!(
    |_, __values| unsafe { (!__values[0].__eq__(&__values[1])).__to_register__() },
    none
);

pub static __VALUE_CALL_LINKAGE: __Linkage = transfer_linkage!(
    |ctx, values| unsafe {
        let call_form_value: &__CallFormValue = values[0].downcast_temp_ref();
        if let Some(linkage) = call_form_value.opt_linkage {
            todo!()
            // match linkage {
            //     __Linkage::Member(_) => todo!(),
            //     __Linkage::SpecificTransfer(linkage) => linkage.fp.0(ctx, &mut values[1..]),
            //     __Linkage::GenericTransfer(_) => todo!(),
            //     __Linkage::Model(_) => todo!(),
            // }
        } else {
            todo!()
        }
    },
    none
);
