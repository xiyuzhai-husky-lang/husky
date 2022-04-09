use std::collections::HashMap;

use vm::{EvalResult, EvalValue};

use super::*;

#[derive(Default, Debug)]
pub struct FeatureSheet<'eval> {
    values: HashMap<FeaturePtr, EvalResult<'eval>>,
}

impl<'eval> FeatureSheet<'eval> {
    pub(crate) fn cached_value(&self, feature: FeaturePtr) -> Option<EvalResult<'eval>> {
        self.values
            .get(&feature)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn cache(
        &mut self,
        feature: FeaturePtr,
        value: EvalResult<'eval>,
    ) -> EvalResult<'eval> {
        let result = unsafe { share_cached(&value) };
        assert!(self.values.insert(feature, value).is_none());
        result
    }

    // pub(crate) fn resolve_class_call(
    //     &mut self,
    //     db: &dyn FeatureQueryGroup,
    //     eval_id: FeatureEvalId,
    //     entity: &Arc<Entity>,
    //     opds: &[Arc<FeatureExpr>],
    // ) -> Object {
    //     if let Some(object) = self.resolved_class_calls.get(&eval_id) {
    //         return object.clone();
    //     }
    //     let object = match entity.kind() {
    //         EntityKind::Ty(ty) => match ty.kind {
    //             TyDefnKind::Record {
    //                 ref field_vars,
    //                 ref field_features,
    //             } => {
    //                 assert!(field_vars.len() == opds.len());
    //                 let field_features = field_features
    //                     .iter()
    //                     .map(|(_ident, defn)| {
    //                         FeatureBlock::new(db, &defn.stmts, &[], db.features())
    //                     })
    //                     .collect();
    //                 Object {
    //                     field_vars: opds.to_vec(),
    //                     field_features,
    //                 }
    //             }
    //             _ => panic!(),
    //         },
    //         _ => panic!(),
    //     };
    //     self.resolved_class_calls.insert(eval_id, object.clone());
    //     object
    // }
}

unsafe fn share_cached<'eval>(cached: &EvalResult<'eval>) -> EvalResult<'eval> {
    Ok(match cached {
        Ok(value) => match value {
            EvalValue::Primitive(value) => EvalValue::Primitive(*value),
            EvalValue::Boxed(value) => EvalValue::GlobalRef(&*value.pointer()),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::Undefined => EvalValue::Undefined,
        },
        Err(error) => Err(error.clone())?,
    })
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &FeatureSheet<'cache>;
}
