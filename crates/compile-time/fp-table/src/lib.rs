use std::collections::HashMap;

use check_utils::*;
use sync_utils::ARwLock;
use vm::{BoxedValue, ElemAccessFp, EvalValue, RoutineFp, StackValue, VMResult};
use vm::{EntityUid, MembAccessFp};
use word::CustomIdentifier;

pub trait HasFpTable {
    fn fp_table(&self) -> &FpTable;
}

#[derive(Debug, Default, Clone)]
pub struct FpTable {
    routine_fps: ARwLock<HashMap<RoutineKey, RoutineFp>>,
    memb_access_fps: ARwLock<HashMap<MembAccessKey, MembAccessFp>>,
    elem_access_fps: ARwLock<HashMap<EntityUid, ElemAccessFp>>,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MembAccessKey {
    this_ty_uid: EntityUid,
    memb_ident: CustomIdentifier,
}

impl FpTable {
    pub fn vec_constructor(&self, element_ty_uid: EntityUid) -> RoutineFp {
        let routine_key = RoutineKey::VecConstructor { element_ty_uid };
        if let Some(compiled_routine) = self.routine_fp(routine_key) {
            compiled_routine
        } else {
            RoutineFp {
                call: construct_virtual_vec,
                nargs: 0,
            }
        }
    }

    pub fn struct_constructor(&self, ty_uid: EntityUid) -> Option<RoutineFp> {
        self.routine_fp(RoutineKey::StructConstructor { ty_uid })
    }

    pub fn entity_routine(&self, routine_uid: EntityUid) -> Option<RoutineFp> {
        self.routine_fp(RoutineKey::EntityRoutine { routine_uid })
    }

    pub fn memb_access(
        &self,
        this_ty_uid: EntityUid,
        memb_ident: CustomIdentifier,
    ) -> Option<MembAccessFp> {
        self.memb_access_fp(MembAccessKey {
            this_ty_uid,
            memb_ident,
        })
    }

    pub fn memb_routine(
        &self,
        this_ty_uid: EntityUid,
        memb_ident: CustomIdentifier,
    ) -> Option<RoutineFp> {
        self.routine_fp(RoutineKey::MembRoutine {
            this_ty_uid,
            memb_ident,
        })
    }

    fn routine_fp(&self, key: RoutineKey) -> Option<RoutineFp> {
        self.routine_fps
            .read(|entries| entries.get(&key).map(|compiled_routine| *compiled_routine))
    }

    fn memb_access_fp(&self, key: MembAccessKey) -> Option<MembAccessFp> {
        self.memb_access_fps
            .read(|entries| entries.get(&key).map(|compiled_routine| *compiled_routine))
    }

    fn elem_access_fp(&self, key: EntityUid) -> Option<ElemAccessFp> {
        self.elem_access_fps
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
