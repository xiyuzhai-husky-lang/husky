use super::*;
use vm::EvalValue;
use word::IdentDict;

#[derive(Debug, Clone)]
pub struct Object {
    pub fields: Vec<Arc<FeatureExpr>>,
    pub field_features: Vec<Arc<FeatureLazyBlock>>,
}

impl<'eval> Into<EvalValue<'eval>> for Object {
    fn into(self) -> EvalValue<'eval> {
        todo!()
    }
}
