mod exec;
mod query;

use husky_ethereal_term::EtherealTerm;
use husky_print_utils::ps;
use husky_text::ModuleRange;
use husky_word::Ident;
use indexmap::IndexMap;
pub use query::InterpreterQueryGroup;

use crate::*;

pub struct Interpreter<'a, 'eval: 'a> {
    db: &'a dyn InterpreterQueryGroup,
    opt_ctx: Option<&'a dyn __EvalContext<'eval>>,
    stack: VMStack<'eval>,
    pub(crate) history: History<'eval>,
    opt_snapshot_saved: Option<StackSnapshot<'eval>>,
    pub(crate) frames: Vec<LoopFrameData<'eval>>,
    variable_mutations: IndexMap<VMStackIdx, (Ident, ModuleRange, EtherealTerm)>,
    vm_config: &'a VMConfig,
}

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn try_new(
        db: &'temp dyn InterpreterQueryGroup,
        opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
        argument_iter: impl Iterator<Item = __VMResult<__Register<'eval>>>,
        vm_config: &'temp VMConfig,
    ) -> __VMResult<Interpreter<'temp, 'eval>> {
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
        opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
        argument_iter: impl Iterator<Item = __Register<'eval>>,
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
        opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
        stack: VMStack<'eval>,
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
    ) -> __VMResult<__Register<'eval>> {
        match self.exec_all(sheet, mode) {
            VMControl::None => {
                panic!("no return from eval_instructions")
            }
            VMControl::Return(result) => Ok(result),
            VMControl::Break => todo!(),
            VMControl::Err(e) => Err(e),
        }
    }

    fn push_new_virtual_struct(&mut self, ty: EtherealTerm, fields: &[Ident]) {
        let parameters = self.stack.drain(fields.len().try_into().unwrap());
        let value =
            unsafe { DeprecatedVirtualStruct::new_struct(ty, parameters, fields).__to_register() };
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
        varname: Ident,
        range: ModuleRange,
        ty: EtherealTerm,
    ) {
        self.variable_mutations
            .insert(stack_idx, (varname, range, ty));
    }

    fn collect_block_mutations(&mut self) -> (StackSnapshot<'eval>, Vec<MutationData<'eval>>) {
        let snapshot = std::mem::take(&mut self.opt_snapshot_saved).expect("bug");
        let mutations = std::mem::take(&mut self.variable_mutations)
            .iter()
            .filter_map(|(stack_idx, (_varname, _range, _ty))| {
                let stack_idx = *stack_idx;
                if stack_idx.raw() < snapshot.len().min(self.stack.len()) {
                    todo!()
                    // Some(MutationData {
                    //     file: *file,
                    //     range: *range,
                    //     kind: MutationDataVariant::Block {
                    //         stack_idx,
                    //         varname: *varname,
                    //     },
                    //     ty: *ty,
                    //     before: Some(snapshot[stack_idx].snapshot()),
                    //     after: self.stack.eval(stack_idx),
                    // })
                } else {
                    None
                }
            })
            .collect();
        (snapshot, mutations)
    }
}
