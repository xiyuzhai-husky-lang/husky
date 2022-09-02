mod __rust_code_gen__;
mod boosting;
mod naive;
mod normalize;

pub use boosting::*;
pub use naive::*;
pub use normalize::*;

use husky_dev_utils::static_dev_src;
use husky_dev_utils::*;
use husky_liason_semantics::*;
use husky_print_utils::p;
use husky_static_defn::*;
use husky_static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};
use husky_static_visualizer::StaticVisualTy;
use husky_trace_protocol::Label;
use husky_vm::{
    Model, __Linkage, __ModelLinkage, __Register, __Registrable, __RegistrableSafe, __StaticInfo,
    __VMResult, __VirtualEnum, __I32_VTABLE, __VIRTUAL_ENUM_VTABLE, __VIRTUAL_FUNCTION_VTABLE,
};
use husky_vm_register_method::VMRegisterMethodX;
use std::{collections::HashMap, sync::Arc, time::Instant};

static_mod! { models = { naive, normalize, boosting } }
