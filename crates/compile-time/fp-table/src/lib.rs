use std::collections::HashMap;

use check_utils::*;
use sync_utils::ARwLock;
use vm::EntityUid;
use vm::{BoxedValue, EvalValue, RoutineFp, StackValue, VMResult};
use word::CustomIdentifier;

pub trait HasFpTable {
    fn fp_table(&self) -> &FpTable;
}

#[derive(Debug, Default, Clone)]
pub struct FpTable {
    compiled_rust_calls: ARwLock<HashMap<RoutineKey, RoutineFp>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RoutineKey {
    VecConstructor {
        element_ty_uid: EntityUid,
    },
    StructConstructor {
        ty_uid: EntityUid,
    },
    EntityRoutine {
        routine_uid: EntityUid,
    },
    MembRoutine {
        this_ty_uid: EntityUid,
        memb_ident: CustomIdentifier,
    },
}

impl FpTable {
    pub fn vec_constructor(&self, element_ty_uid: EntityUid) -> RoutineFp {
        let routine_key = RoutineKey::VecConstructor { element_ty_uid };
        if let Some(compiled_routine) = self.routine_fp_by_key(routine_key) {
            compiled_routine
        } else {
            RoutineFp {
                call: construct_virtual_vec,
                nargs: 0,
            }
        }
    }

    pub fn struct_constructor(&self, ty_uid: EntityUid) -> Option<RoutineFp> {
        self.routine_fp_by_key(RoutineKey::StructConstructor { ty_uid })
    }

    pub fn entity_routine(&self, routine_uid: EntityUid) -> Option<RoutineFp> {
        self.routine_fp_by_key(RoutineKey::EntityRoutine { routine_uid })
    }

    pub fn memb_routine(
        &self,
        this_ty_uid: EntityUid,
        memb_ident: CustomIdentifier,
    ) -> Option<RoutineFp> {
        self.routine_fp_by_key(RoutineKey::MembRoutine {
            this_ty_uid,
            memb_ident,
        })
    }

    fn routine_fp_by_key(&self, key: RoutineKey) -> Option<RoutineFp> {
        self.compiled_rust_calls
            .read(|entries| entries.get(&key).map(|compiled_routine| *compiled_routine))
    }
}

fn construct_virtual_vec<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 0);
    Ok(StackValue::Boxed(BoxedValue::new(
        Vec::<EvalValue<'eval>>::new(),
    )))
}
