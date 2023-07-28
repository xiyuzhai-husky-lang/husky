use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __LinkageGroup)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __LinkageGroup)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_input_major::find_connected_components",
    },
    transfer_linkage!(
        {
            unsafe fn __wrapper<'eval>(
                __arguments: &mut [__Register<'eval>],
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            ) -> __Register<'eval> {
                let binary_image: &cv::datasets::mnist::BinaryImage28 = __arguments[0].downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE);
                find_connected_components(binary_image).to_register()
            }
            __wrapper
        },
        some base find_connected_components as fn(&'static cv::datasets::mnist::BinaryImage28) -> i32
    ),
)];
