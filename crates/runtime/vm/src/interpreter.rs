mod exec;
mod query;

use std::collections::HashMap;

pub use query::InterpreterQueryGroup;
use word::{CustomIdentifier, Identifier};

use crate::*;

pub struct Interpreter<'stack, 'eval: 'stack> {
    db: &'stack dyn InterpreterQueryGroup,
    stack: VMStack<'stack, 'eval>,
    pub(crate) history: History<'eval>,
    snapshot: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    mutations: HashMap<StackIdx, Identifier>,
}

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn try_new(
        db: &'stack dyn InterpreterQueryGroup,
        iter: impl Iterator<Item = VMResult<StackValue<'stack, 'eval>>>,
    ) -> VMResult<Interpreter<'stack, 'eval>> {
        Ok(Self {
            db,
            stack: VMStack::try_new(iter)?,
            history: Default::default(),
            snapshot: None,
            frames: vec![],
            mutations: Default::default(),
        })
    }

    pub(crate) fn new(
        db: &'stack dyn InterpreterQueryGroup,
        values: impl Into<VMStack<'stack, 'eval>>,
    ) -> Interpreter<'stack, 'eval> {
        Self {
            db,
            stack: values.into(),
            history: Default::default(),
            snapshot: None,
            frames: vec![],
            mutations: Default::default(),
        }
    }

    pub(crate) fn eval_instructions(
        &mut self,
        instructions: &[Instruction],
        mode: Mode,
    ) -> EvalResult<'eval> {
        match self.exec_all(instructions, mode) {
            VMControl::None => {
                panic!("no return from eval_instructions")
            }
            VMControl::Return(result) => Ok(result),
            VMControl::Break => todo!(),
            VMControl::Err(_) => todo!(),
        }
    }

    fn call_compiled(&mut self, f: Linkage) -> VMResult<()> {
        let result = (f.call)(&mut self.stack.drain(f.nargs))?;
        self.stack.push(result.into());
        Ok(())
    }

    fn routine_call_interpreted(
        &mut self,
        instructions: &[Instruction],
        nargs: u8,
    ) -> VMResult<()> {
        todo!()
    }

    fn new_virtual_struct(&mut self, field_vars: &[FieldContract]) -> VMResult<()> {
        let inputs = self.stack.drain(field_vars.len().try_into().unwrap());
        self.stack
            .push(VirtualTy::new_struct(inputs, field_vars).into());
        Ok(())
    }

    fn take_snapshot(&mut self) {
        if let Some(_) = self.snapshot {
            panic!()
        }
        self.snapshot = Some(self.stack.snapshot());
    }

    fn record_mutation(&mut self, stack_idx: StackIdx, varname: Identifier) {
        self.mutations.insert(stack_idx, varname);
    }

    fn collect_mutations(&mut self) -> (StackSnapshot<'eval>, Vec<MutationData<'eval>>) {
        let snapshot = std::mem::take(&mut self.snapshot).expect("bug");
        let mutations = std::mem::take(&mut self.mutations)
            .iter()
            .map(|(stack_idx, varname)| {
                let stack_idx = *stack_idx;
                MutationData {
                    varname: *varname,
                    before: snapshot[stack_idx].clone(),
                    after: self.stack.snapshot_value(stack_idx),
                }
            })
            .collect();
        (snapshot, mutations)
    }
}
