mod __rust_code_gen__;
mod boosting;
mod naive;
mod narrow;
mod normalize;
mod utils;

pub use boosting::*;
pub use naive::*;
pub use narrow::*;
pub use normalize::*;

use husky_dev_utils::static_dev_src;
use husky_dev_utils::*;
use husky_liason_semantics::*;

use husky_static_defn::*;
use husky_static_defn::{static_mod, EntityStaticDefn, EntityStaticDefnVariant};

use husky_trace_protocol_old::Label;
use husky_vm::{
    Model, __LinkageGroup, __ModelLinkageGroup, __Register, __Registrable, __RegistrableSafe,
    __StaticInfo, __VMResult, __VirtualEnum, __VIRTUAL_ENUM_VTABLE,
};

use std::collections::HashMap;

static_mod! { models = { naive, normalize, boosting, narrow } }
