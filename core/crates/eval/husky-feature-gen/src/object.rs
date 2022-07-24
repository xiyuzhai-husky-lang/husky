use super::*;
use vm::__EvalValue;

#[derive(Debug, Clone)]
pub struct Object {
    pub fields: Vec<Arc<FeatureExpr>>,
    pub field_features: Vec<Arc<FeatureLazyBlock>>,
}

impl<'eval> Into<__Register<'eval>> for Object {
    fn into(self) -> __Register<'eval> {
        todo!()
    }
}
