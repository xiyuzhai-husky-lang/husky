mod exec;
mod query;

use std::collections::HashMap;

use entity_route::EntityRoutePtr;
use file::FilePtr;
use indexmap::IndexMap;
pub use query::InterpreterQueryGroup;
use text::TextRange;
use word::{CustomIdentifier, Identifier};

use crate::*;

pub struct Interpreter<'stack, 'eval: 'stack> {
    db: &'stack dyn InterpreterQueryGroup,
    stack: VMStack<'stack, 'eval>,
    pub(crate) history: History<'eval>,
    opt_snapshot_saved: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    variable_mutations: IndexMap<StackIdx, (Identifier, FilePtr, TextRange, EntityRoutePtr)>,
}

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn try_new(
        db: &'stack dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = VMRuntimeResult<StackValue<'stack, 'eval>>>,
    ) -> VMRuntimeResult<Interpreter<'stack, 'eval>> {
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
        db: &'stack dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = StackValue<'stack, 'eval>>,
        has_this: bool,
    ) -> Interpreter<'stack, 'eval> {
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
        db: &'stack dyn InterpreterQueryGroup,
        prestack: impl Into<VMStack<'stack, 'eval>>,
    ) -> Interpreter<'stack, 'eval> {
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

    fn new_virtual_struct(
        &mut self,
        fields: &[(CustomIdentifier, FieldLiason)],
    ) -> VMRuntimeResult<()> {
        let inputs = self.stack.drain(fields.len().try_into().unwrap());
        let value = VirtualTy::new_struct(inputs, fields).into();
        self.stack.push(value);
        Ok(())
    }

    fn save_snapshot(&mut self) {
        if let Some(_) = self.opt_snapshot_saved {
            panic!()
        }
        self.opt_snapshot_saved = Some(self.stack.snapshot());
    }

    fn record_mutation(
        &mut self,
        stack_idx: StackIdx,
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
