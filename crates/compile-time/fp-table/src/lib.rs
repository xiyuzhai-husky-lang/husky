use std::collections::HashMap;

use check_utils::*;
use sync_utils::ARwLock;
use vm::EntityUid;
use vm::{BoxedValue, CompiledRustCall, EvalValue, StackValue, VMResult};

pub trait HasFpTable {
    fn fp_table(&self) -> &FpTable;
}

#[derive(Debug, Default, Clone)]
pub struct FpTable {
    compiled_rust_calls: ARwLock<HashMap<CallKey, CompiledRustCall>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CallKey {
    VecConstructor { element_ty_uid: EntityUid },
    StructConstructor { ty_uid: EntityUid },
    Routine { routine_uid: EntityUid },
}

impl FpTable {
    pub fn vec_constructor(&self, element_ty_uid: EntityUid) -> CompiledRustCall {
        let routine_key = CallKey::VecConstructor { element_ty_uid };
        if let Some(compiled_routine) = self.compiled_rust_call(routine_key) {
            compiled_routine
        } else {
            CompiledRustCall {
                call: construct_virtual_vec,
            }
        }
    }

    pub fn struct_constructor(&self, ty_uid: EntityUid) -> Option<CompiledRustCall> {
        self.compiled_rust_call(CallKey::StructConstructor { ty_uid })
    }

    pub fn routine(&self, routine_uid: EntityUid) -> Option<CompiledRustCall> {
        self.compiled_rust_call(CallKey::Routine { routine_uid })
    }

    fn compiled_rust_call(&self, key: CallKey) -> Option<CompiledRustCall> {
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
