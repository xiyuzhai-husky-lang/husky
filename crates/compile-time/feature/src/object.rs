use super::*;
use vm::EvalValue;
use word::IdentMap;

#[derive(Debug, Clone)]
pub struct Object {
    pub memb_vars: Vec<Arc<FeatureExpr>>,
    pub memb_features: Vec<Arc<FeatureBlock>>,
}

impl<'eval> Into<EvalValue<'eval>> for Object {
    fn into(self) -> EvalValue<'eval> {
        todo!()
    }
}
