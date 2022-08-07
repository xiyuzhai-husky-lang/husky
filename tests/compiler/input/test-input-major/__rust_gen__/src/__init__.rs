use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[(
    __StaticLinkageKey::Routine {
        route: "test_input_major::find_connected_components",
    },
    __Linkage::Transfer(__LinkageFp {
        dev_src: static_dev_src!(),
        wrapper: {
            unsafe fn __wrapper<'eval>(
                __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                __arguments: &mut [__Register<'eval>],
            ) -> __Register<'eval> {
                let binary_image: &domains::ml::datasets::cv::mnist::BinaryImage28 =
                    __arguments[0].downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE);
                find_connected_components(binary_image).to_register()
            }
            __wrapper
        },
        opt_fp: Some(find_connected_components as *const ()),
    }),
)];
