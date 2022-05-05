mod exec_call;
mod exec_interpret_call;
mod exec_loop;
mod exec_primitive_opn;

use crate::{history::HistoryEntry, *};
use print_utils::p;

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn exec_all(
        &mut self,
        instructions: &[Instruction],
        mode: Mode,
    ) -> VMControl<'eval> {
        for ins in instructions {
            let control = match ins.kind {
                InstructionKind::PushVariable {
                    binding,
                    stack_idx,
                    range,
                    ty,
                } => {
                    let value = self.stack.push_variable(stack_idx, binding);
                    match mode {
                        Mode::Fast => (),
                        Mode::TrackMutation => match binding {
                            Binding::RefMut => {
                                self.record_mutation(stack_idx, ins.src.file(), range, ty)
                            }
                            _ => (),
                        },
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: value.snapshot(),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionKind::PushPrimitiveLiteral(value) => {
                    self.stack.push(value.into());
                    VMControl::None
                }
                InstructionKind::RoutineCallCompiled { linkage } => {
                    let control = self.call_compiled(linkage).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: self.stack.top_snapshot(),
                            },
                        ),
                    }
                    control
                }
                InstructionKind::PrimitiveOpn { opn, .. } => {
                    self.exec_primitive_opn(opn, mode, ins).into()
                }
                InstructionKind::RoutineCallInterpreted { routine, nargs } => {
                    let instruction_sheet = self.db.entity_opt_instruction_sheet_by_uid(routine);
                    let control = self
                        .routine_call_interpreted(&instruction_sheet.unwrap().instructions, nargs)
                        .into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => todo!(),
                    };
                    control
                }
                InstructionKind::NewVirtualStruct {
                    fields: ref field_vars,
                } => {
                    let control = self.new_virtual_struct(field_vars).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => todo!(),
                    };
                    control
                }
                InstructionKind::Return => VMControl::Return(self.stack.pop().into_eval()),
                InstructionKind::Loop {
                    ref body,
                    loop_kind,
                } => match mode {
                    Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
                    Mode::TrackMutation => self.exec_loop_tracking_mutation(loop_kind, body).into(),
                    Mode::TrackHistory => {
                        self.take_snapshot();
                        let control = self.exec_loop_tracking_mutation(loop_kind, body).into();
                        let (snapshot, mutations) = self.collect_mutations();
                        self.history.write(
                            ins,
                            HistoryEntry::loop_entry(&control, snapshot, body.clone(), mutations),
                        );
                        control
                    }
                },
                InstructionKind::BreakIfFalse => {
                    let control = if !self.stack.top().as_primitive().to_bool().unwrap() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    self.stack.pop();
                    control
                }
                InstructionKind::FieldAccessCompiled {
                    linkage: field_access_fp,
                } => todo!(),
                InstructionKind::FieldAccessInterpreted {
                    field_idx,
                    contract,
                } => {
                    let this = self.stack.pop();
                    self.stack
                        .push(this.field_var(field_idx as usize, contract));
                    VMControl::None
                }
                InstructionKind::Assert => {
                    let is_condition_satisfied = self.stack.pop().as_primitive().to_bool().unwrap();
                    if !is_condition_satisfied {
                        todo!()
                    } else {
                        VMControl::None
                    }
                }
                InstructionKind::Break => VMControl::Break,
                InstructionKind::BranchGroup { .. } => todo!(),
            };
            match control {
                VMControl::None => (),
                VMControl::Break | VMControl::Return(_) | VMControl::Err(_) => return control,
            }
        }
        VMControl::None
    }

    pub(crate) fn exec_linkage(&mut self, code: Linkage) -> EvalResult<'eval> {
        todo!()
    }
}
