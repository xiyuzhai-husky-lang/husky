use std::collections::HashMap;

use vm::{EvalRef, EvalResult, EvalValue};
use word::CustomIdentifier;

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet<'eval> {
    values: Mutex<HashMap<EvalKey<'eval>, EvalValueResult<'eval>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EvalKey<'eval> {
    Feature(FeaturePtr),
    StructDerivedField {
        parent: EvalRef<'eval>,
        field_ident: CustomIdentifier,
    },
}

unsafe impl<'eval> Send for EvalKey<'eval> {}

unsafe impl<'eval> Sync for EvalKey<'eval> {}

impl<'eval> EvalSheet<'eval> {
    pub(crate) fn cached_value(&self, eval_key: EvalKey<'eval>) -> Option<EvalValueResult<'eval>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey<'eval>,
        value: EvalValueResult<'eval>,
    ) -> EvalValueResult<'eval> {
        let mut values = self.values.lock().unwrap();
        if !values.contains_key(&eval_key) {
            let result = unsafe { share_cached(&value) };
            assert!(values.insert(eval_key, value).is_none());
            result
        } else {
            value
        }
    }

    pub(crate) fn cache(
        &self,
        eval_key: EvalKey<'eval>,
        value: EvalValueResult<'eval>,
    ) -> EvalValueResult<'eval> {
        let result = unsafe { share_cached(&value) };
        assert!(self
            .values
            .lock()
            .unwrap()
            .insert(eval_key, value)
            .is_none());
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
    //             TyKind::Record {
    //                 ref fields,
    //                 ref field_features,
    //             } => {
    //                 assert!(fields.len() == opds.len());
    //                 let field_features = field_features
    //                     .iter()
    //                     .map(|(_ident, defn)| {
    //                         FeatureBlock::new(db, &defn.stmts, &[], db.features())
    //                     })
    //                     .collect();
    //                 Object {
    //                     fields: opds.to_vec(),
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

unsafe fn share_cached<'eval>(cached: &EvalValueResult<'eval>) -> EvalValueResult<'eval> {
    Ok(match cached {
        Ok(value) => match value {
            EvalValue::Copyable(value) => EvalValue::Copyable(*value),
            EvalValue::Owned(value) => EvalValue::EvalRef(EvalRef(&*value.any_ptr())),
            EvalValue::EvalRef(value) => EvalValue::EvalRef(*value),
            EvalValue::EvalPure(value) => EvalValue::EvalPure(value.clone()),
            EvalValue::Undefined => EvalValue::Undefined,
        },
        Err(error) => Err(error.clone())?,
    })
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
