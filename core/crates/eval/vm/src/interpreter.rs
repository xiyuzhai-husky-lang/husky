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
    verbose: bool,
}

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn try_new(
        db: &'temp dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = EvalResult<TempValue<'temp, 'eval>>>,
        verbose: bool,
    ) -> EvalResult<Interpreter<'temp, 'eval>> {
        Ok(Self {
            db,
            stack: VMStack::try_new(argument_iter)?,
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            verbose,
        })
    }

    pub(crate) fn new(
        db: &'temp dyn InterpreterQueryGroup,
        argument_iter: impl Iterator<Item = TempValue<'temp, 'eval>>,
        has_this: bool,
        verbose: bool,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            stack: VMStack::new(argument_iter),
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            verbose,
        }
    }

    pub(crate) fn from_prestack(
        db: &'temp dyn InterpreterQueryGroup,
        prestack: impl Into<VMStack<'temp, 'eval>>,
        verbose: bool,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            stack: prestack.into(),
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            verbose,
        }
    }

    pub(crate) fn eval_instructions(
        &mut self,
        sheet: &InstructionSheet,
        mode: Mode,
    ) -> EvalValueResult<'eval> {
        match self.exec_all(sheet, mode) {
            VMControl::None => {
                panic!("no return from eval_instructions")
            }
            VMControl::Return(result) => Ok(result),
            VMControl::Break => todo!(),
            VMControl::Err(e) => Err(e.into()),
        }
    }

    fn new_virtual_struct(&mut self, ty: EntityRoutePtr, fields: &[CustomIdentifier]) {
        let parameters = self.stack.drain(fields.len().try_into().unwrap());
        let value = VirtualStruct::new_struct(ty, parameters, fields).into();
        self.stack.push(value)
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
            .filter_map(|(stack_idx, (varname, file, range, ty))| {
                let stack_idx = *stack_idx;
                if stack_idx.raw() < snapshot.len().min(self.stack.len()) {
                    Some(MutationData {
                        file: *file,
                        range: *range,
                        kind: MutationDataVariant::Block {
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
