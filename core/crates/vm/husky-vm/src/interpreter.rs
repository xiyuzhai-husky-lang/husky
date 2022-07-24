mod exec;
mod query;

use std::collections::HashMap;

use husky_entity_route::EntityRoutePtr;
use husky_file::FilePtr;
use husky_print_utils::{p, ps};
use husky_text::TextRange;
use indexmap::IndexMap;
pub use query::InterpreterQueryGroup;
use word::{CustomIdentifier, Identifier};

use crate::*;

pub struct Interpreter<'temp, 'eval: 'temp> {
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&'temp dyn EvalContextDeprecated<'eval>>,
    stack: VMStack<'temp, 'eval>,
    pub(crate) history: History<'eval>,
    opt_snapshot_saved: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    variable_mutations: IndexMap<VMStackIdx, (Identifier, FilePtr, TextRange, EntityRoutePtr)>,
    vm_config: &'temp VMConfig,
}

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn try_new(
        db: &'temp dyn InterpreterQueryGroup,
        opt_ctx: Option<&'temp dyn EvalContextDeprecated<'eval>>,
        argument_iter: impl Iterator<Item = __EvalResult<__TempValue<'temp, 'eval>>>,
        vm_config: &'temp VMConfig,
    ) -> __EvalResult<Interpreter<'temp, 'eval>> {
        Ok(Self {
            db,
            opt_ctx,
            stack: VMStack::try_new(argument_iter)?,
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            vm_config,
        })
    }

    pub(crate) fn new(
        db: &'temp dyn InterpreterQueryGroup,
        opt_ctx: Option<&'temp dyn EvalContextDeprecated<'eval>>,
        argument_iter: impl Iterator<Item = __TempValue<'temp, 'eval>>,
        has_this: bool,
        vm_config: &'temp VMConfig,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            opt_ctx,
            stack: VMStack::new(argument_iter),
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            vm_config,
        }
    }

    pub(crate) fn from_stack(
        db: &'temp dyn InterpreterQueryGroup,
        opt_ctx: Option<&'temp dyn EvalContextDeprecated<'eval>>,
        stack: VMStack<'temp, 'eval>,
        vm_config: &'temp VMConfig,
    ) -> Interpreter<'temp, 'eval> {
        Self {
            db,
            opt_ctx,
            stack,
            history: Default::default(),
            opt_snapshot_saved: None,
            frames: vec![],
            variable_mutations: Default::default(),
            vm_config,
        }
    }

    pub(crate) fn eval_instructions(
        &mut self,
        sheet: &InstructionSheet,
        mode: Mode,
    ) -> __EvalValueResult<'eval> {
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
