use husky_defn_head::Parameter;
use husky_static_defn::{EntityStaticDefn, EntityStaticDefnVariant};
use husky_term::Ty;
use husky_vm::__Linkage;
use map_collect::MapCollect;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeCallDefn {
    pub parameters: Arc<Vec<Parameter>>,
    pub output_ty: Ty,
    pub opt_linkage: Option<__Linkage>,
}
