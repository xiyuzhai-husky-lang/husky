mod exec_call;
mod exec_interpret_call;
mod exec_loop;
mod exec_primitive_opn;

use crate::{history::HistoryEntry, *};

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn exec_all(
        &mut self,
        instructions: &[Instruction],
        mode: Mode,
    ) -> VMControl<'eval> {
        for ins in instructions {
            let control = match ins.kind {
                InstructionKind::PushVariable {
                    contract,
                    stack_idx,
                } => {
                    let value = self.stack.push_variable(stack_idx, contract);
                    match mode {
                        Mode::Fast => (),
                        Mode::Debug => {
                            let snapshot = value.snapshot();
                            self.history
                                .write(ins, HistoryEntry::NonVoidExpr { output: snapshot })
                        }
                    }
                    VMControl::None
                }
                InstructionKind::PushPrimitiveLiteral(value) => {
                    self.stack.push(value.into());
                    VMControl::None
                }
                InstructionKind::RoutineCallCompiled { fp } => {
                    let control = self.call_compiled(fp).into();
                    match mode {
                        Mode::Fast => (),
                        Mode::Debug => todo!(),
                    }
                    control
                }
                InstructionKind::PrimitiveOpn(opn) => {
                    self.exec_primitive_opn(opn, mode, ins).into()
                }
                InstructionKind::RoutineCallInterpreted { routine, nargs } => {
                    let instruction_sheet = self.db.entity_instruction_sheet_by_uid(routine);
                    let control = self
                        .routine_call_interpreted(&instruction_sheet.instructions, nargs)
                        .into();
                    match mode {
                        Mode::Fast => (),
                        Mode::Debug => todo!(),
                    };
                    control
                }
                InstructionKind::NewVirtualStruct { ref memb_vars } => {
                    let control = self.new_virtual_struct(memb_vars).into();
                    match mode {
                        Mode::Fast => (),
                        Mode::Debug => todo!(),
                    };
                    control
                }
                InstructionKind::Return => VMControl::Return(self.stack.pop().into_eval()),
                InstructionKind::Init(init_kind) => self.init(init_kind, mode).into(),
                InstructionKind::Loop {
                    ref body,
                    loop_kind,
                } => match mode {
                    Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
                    Mode::Debug => {
                        let stack_snapshot = self.stack.snapshot();
                        let control = self.exec_loop_fast(loop_kind, body).into();
                        self.history.write(
                            ins,
                            HistoryEntry::loop_entry(&control, stack_snapshot, body.clone()),
                        );
                        control
                    }
                },
                InstructionKind::BreakIfFalse => {
                    let control = if !self.stack.top().as_primitive().unwrap().to_bool().unwrap() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    self.stack.pop();
                    control
                }
                InstructionKind::MembAccessCompiled { memb_access_fp } => todo!(),
                InstructionKind::MembAccessInterpreted { memb_idx, contract } => {
                    let this = self.stack.pop();
                    self.stack.push(this.memb_var(memb_idx as usize, contract));
                    VMControl::None
                }
            };
            match control {
                VMControl::None => (),
                VMControl::Break | VMControl::Return(_) | VMControl::Err(_) => return control,
            }
        }
        VMControl::None
    }

    pub(crate) fn exec_code(&mut self, code: RoutineFp) -> EvalResult<'eval> {
        todo!()
    }

    fn init(&mut self, init_kind: InitKind, mode: Mode) -> VMResult<()> {
        // match init_kind {
        //     InitKind::Let => todo!(),
        //     InitKind::Var => todo!(),
        //     InitKind::Decl => todo!(),
        // }
        match self.stack.top_mut(mode) {
            StackValue::Primitive(_)
            | StackValue::Boxed(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_) => (),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { .. } => todo!(),
            StackValue::Moved => todo!(),
        }
        Ok(())
    }

    // fn exec(&mut self, ins_kind: &InstructionKind) -> VMControl<'eval> {
    //     match ins_kind {
    //         InstructionKind::PushVariable {
    //             contract,
    //             stack_idx: idx,
    //         } => {
    //             self.stack.push(self.stack.value(*idx).bind(*contract)?);
    //             Ok(VMControl::Normal)
    //         }
    //         InstructionKind::PushPrimitiveLiteral(value) => {
    //             self.stack.push(value.into());
    //             Ok(VMControl::Normal)
    //         }
    //         InstructionKind::Call {
    //             ref compiled,
    //             nargs,
    //         } => {
    //             self.call(compiled, *nargs)?;
    //             Ok(VMControl::Normal)
    //         }
    //         InstructionKind::PrimitiveOpn(opn) => {
    //             self.exec_primitive_opn(*opn)?;
    //             Ok(VMControl::Normal)
    //         }
    //         InstructionKind::InterpretCall(ref Instructions) => self.exec_all(Instructions),
    //         InstructionKind::Return => {
    //             Ok(VMControl::Return(self.stack.pop().into_eval()?))
    //         }
    //         InstructionKind::Init(init_kind) => {
    //             self.init(*init_kind)?;
    //             Ok(VMControl::Normal)
    //         }
    //         InstructionKind::Loop { body, loop_kind } => todo!(),
    //     }
    // }
}
