use crate::*;
use dev_utils::*;

pub static __EQ_LINKAGE: __Linkage = specific_transfer_linkage!(
    |_, __values| {
        __TempValue::Copyable((__values[0].any_ref().__equal_any(__values[1].any_ref())).into())
    },
    none
);

pub static __NEQ_LINKAGE: __Linkage = specific_transfer_linkage!(
    |_, __values| {
        __TempValue::Copyable((!__values[0].any_ref().__equal_any(__values[1].any_ref())).into())
    },
    none
);

pub static __VALUE_CALL_LINKAGE: __Linkage = specific_transfer_linkage!(
    |ctx, values| {
        let call_form_value: &__CallFormValue = values[0].downcast_temp_ref();
        if let Some(linkage) = call_form_value.opt_linkage {
            match linkage {
                __Linkage::Member(_) => todo!(),
                __Linkage::SpecificTransfer(linkage) => linkage.fp.0(ctx, &mut values[1..]),
                __Linkage::GenericTransfer(_) => todo!(),
                __Linkage::Model(_) => todo!(),
            }
        } else {
            todo!()
        }
    },
    none
);
