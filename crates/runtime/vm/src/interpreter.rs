mod exec;
mod query;

use std::collections::HashMap;

use entity_route::EntityRoutePtr;
use file::FilePtr;
pub use query::InterpreterQueryGroup;
use text::TextRange;
use word::{CustomIdentifier, Identifier};

use crate::*;

pub struct Interpreter<'stack, 'eval: 'stack> {
    db: &'stack dyn InterpreterQueryGroup,
    stack: VMStack<'stack, 'eval>,
    pub(crate) history: History<'eval>,
    snapshot: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    mutations: HashMap<StackIdx, (FilePtr, TextRange, EntityRoutePtr)>,
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

    fn call_compiled(&mut self, f: Linkage) -> VMResult<()> {
        let result = (f.call)(&mut self.stack.drain(f.nargs))?;
        self.stack.push(result.into());
        Ok(())
    }

    fn routine_call_interpreted(&mut self, sheet: &InstructionSheet, nargs: u8) -> VMResult<()> {
        let inputs = self.stack.drain(nargs);
        let mut interpreter = Interpreter::new(self.db, inputs);
        self.stack.push(
            interpreter
                .eval_instructions(sheet, Mode::Fast)?
                .into_stack()?,
        );
        Ok(())
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

    fn record_mutation(
        &mut self,
        stack_idx: StackIdx,
        file: FilePtr,
        range: TextRange,
        ty: EntityRoutePtr,
    ) {
        self.mutations.insert(stack_idx, (file, range, ty));
    }

    fn collect_mutations(&mut self) -> (StackSnapshot<'eval>, Vec<MutationData<'eval>>) {
        let snapshot = std::mem::take(&mut self.snapshot).expect("bug");
        let mutations = std::mem::take(&mut self.mutations)
            .iter()
            .map(|(stack_idx, (file, range, ty))| {
                let stack_idx = *stack_idx;
                MutationData {
                    file: *file,
                    range: *range,
                    ty: *ty,
                    before: snapshot[stack_idx].clone(),
                    after: self.stack.snapshot_value(stack_idx),
                }
            })
            .collect();
        (snapshot, mutations)
    }
}
