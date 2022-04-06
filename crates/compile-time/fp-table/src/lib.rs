use std::collections::HashMap;

use check_utils::*;
use entity_route::EntityRoutePtr;
use semantics_entity::EntityUid;
use sync_utils::ARwLock;
use vm::{BoxedValue, CompiledRoutine, EvalValue, StackValue, VMResult};

pub trait HasFpTable {
    fn fp_table(&self) -> &FpTable;
}

#[derive(Debug, Default, Clone)]
pub struct FpTable {
    compiled_routines: ARwLock<HashMap<RoutineKey, CompiledRoutine>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RoutineKey {
    VecConstructor { element_ty_uid: EntityUid },
    StructConstructor { ty_uid: EntityUid },
}

impl FpTable {
    pub fn vec_constructor(&self, element_ty_uid: EntityUid) -> CompiledRoutine {
        let routine_key = RoutineKey::VecConstructor { element_ty_uid };
        if let Some(compiled_routine) = self.compiled_routine(routine_key) {
            compiled_routine
        } else {
            CompiledRoutine {
                call: construct_virtual_vec,
            }
        }
    }

    pub fn struct_constructor(&self, ty_uid: EntityUid) -> Option<CompiledRoutine> {
        self.compiled_routine(RoutineKey::StructConstructor { ty_uid })
    }

    fn compiled_routine(&self, key: RoutineKey) -> Option<CompiledRoutine> {
        self.compiled_routines
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
