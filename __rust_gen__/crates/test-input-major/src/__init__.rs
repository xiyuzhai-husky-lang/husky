use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        routine: "test_input_major::find_connected_components",
    },
    specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let binary_image: &domains::ml::datasets::cv::mnist::BinaryImage28 = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    find_connected_components(binary_image)
                .__take_copyable_dyn())
            }
            __wrapper
        }, some find_connected_components),
)];
