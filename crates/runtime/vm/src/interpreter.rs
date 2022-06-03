mod exec;
mod query;

use std::collections::HashMap;

use entity_route::EntityRoutePtr;
use file::FilePtr;
use indexmap::IndexMap;
use print_utils::{p, ps};
pub use query::InterpreterQueryGroup;
use text::TextRange;
use word::{CustomIdentifier, Identifier};

use crate::*;

pub struct Interpreter<'temp, 'eval: 'temp> {
    db: &'temp dyn InterpreterQueryGroup,
    stack: VMStack<'temp, 'eval>,
    pub(crate) history: History<'eval>,
    opt_snapshot_saved: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    variable_mutations: IndexMap<VMStackIdx, (Identifier, FilePtr, TextRange, EntityRoutePtr)>,
}

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn try_new(
        db: &'temp dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = VMRuntimeResult<TempValue<'temp, 'eval>>>,
    ) -> VMRuntimeResult<Interpreter<'temp, 'eval>> {
        Ok(Self {
            db,
            stack: VMStack::try_new(argument_iter)?,
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
        })
    }

    pub(crate) fn new(
        db: &'temp dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = TempValue<'temp, 'eval>>,
        has_this: bool,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            stack: VMStack::new(argument_iter),
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
        }
    }

    pub(crate) fn from_prestack(
        db: &'temp dyn InterpreterQueryGroup,
        prestack: impl Into<VMStack<'temp, 'eval>>,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            stack: prestack.into(),
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
        }
    }

    pub(crate) fn eval_instructions(
        &mut self,
        sheet: &InstructionSheet,
        mode: Mode,
    ) -> EvalResult<'eval> {
        match self.exec_all(sheet, mode) {
            VMControl::None => {
                panic!("no return from eval_instructions")
            }
            VMControl::Return(result) => Ok(result),
            VMControl::Break => todo!(),
            VMControl::Err(e) => Err(e),
        }
    }

    fn new_virtual_struct(&mut self, fields: &[CustomIdentifier]) -> VMRuntimeResult<()> {
        let parameters = self.stack.drain(fields.len().try_into().unwrap());
        let value = VirtualTy::new_struct(parameters, fields).into();
        self.stack.push(value);
        Ok(())
    }

    fn save_snapshot(&mut self, message: String) {
        if let Some(ref snapshot) = self.opt_snapshot_saved {
            ps!(snapshot.message);
            ps!(message);
            panic!()
        }
        self.opt_snapshot_saved = Some(self.stack.snapshot(message));
    }

    fn record_mutation(
        &mut self,
        stack_idx: VMStackIdx,
        varname: Identifier,
        file: FilePtr,
        range: TextRange,
        ty: EntityRoutePtr,
    ) {
        self.variable_mutations
            .insert(stack_idx, (varname, file, range, ty));
    }

    fn collect_block_mutations(&mut self) -> (StackSnapshot<'eval>, Vec<MutationData<'eval>>) {
        let snapshot = std::mem::take(&mut self.opt_snapshot_saved).expect("bug");
        let mutations = std::mem::take(&mut self.variable_mutations)
            .iter()
            .filter_map(|(stack_idx, (varname, file, _, ty))| {
                let stack_idx = *stack_idx;
                if stack_idx.raw() < snapshot.len().min(self.stack.len()) {
                    Some(MutationData {
                        file: *file,
                        kind: MutationDataKind::Block {
                            stack_idx,
                            varname: *varname,
                        },
                        ty: *ty,
                        before: Some(snapshot[stack_idx].eval()),
                        after: self.stack.eval(stack_idx),
                    })
                } else {
                    None
                }
            })
            .collect();
        (snapshot, mutations)
    }
}
