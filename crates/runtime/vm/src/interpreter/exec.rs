mod exec_call;
mod exec_condition_flow;
mod exec_interpret_call;
mod exec_loop;
mod exec_opr_opn;
mod exec_pattern_match;

use std::iter::zip;

use crate::{history::HistoryEntry, *};
use check_utils::{should, should_eq};
use print_utils::{p, ps};

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(crate) fn exec_all(&mut self, sheet: &InstructionSheet, mode: Mode) -> VMControl<'eval> {
        for ins in &sheet.instructions {
            let control = match ins.kind {
                InstructionKind::PushVariable {
                    binding,
                    stack_idx,
                    range,
                    ty,
                    varname,
                } => {
                    let value = self.stack.push_variable(stack_idx, binding);
                    match mode {
                        Mode::Fast => (),
                        Mode::TrackMutation => match binding {
                            Binding::RefMut => {
                                self.record_mutation(stack_idx, varname, ins.src.file(), range, ty)
                            }
                            _ => (),
                        },
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(value.eval()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionKind::PushPrimitiveLiteral(value) => {
                    self.stack.push(value.into());
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(self.stack.eval_top()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionKind::PushEnumKindLiteral(entity_kind) => {
                    self.stack.push(StackValue::Copyable(entity_kind.into()));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(self.stack.eval_top()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionKind::RoutineCallCompiled { linkage } => {
                    let control = self.call_compiled(linkage).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: match control {
                                    VMControl::Err(ref e) => Err(e.clone()),
                                    _ => Ok(self.stack.eval_top()),
                                },
                            },
                        ),
                    }
                    control
                }
                InstructionKind::OprOpn { opn, .. } => {
                    // sheet.variable_stack.compare_with_vm_stack(&self.stack);
                    self.exec_opr_opn(opn, mode, ins).into()
                }
                InstructionKind::RoutineCallInterpreted { routine, nargs } => {
                    let instruction_sheet = self.db.entity_opt_instruction_sheet_by_uid(routine);
                    let control = self
                        .routine_call_interpreted(&instruction_sheet.unwrap(), nargs)
                        .into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(self.stack.eval_top()),
                            },
                        ),
                    };
                    control
                }
                InstructionKind::NewVirtualStruct { ref fields } => {
                    let control = self.new_virtual_struct(fields).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(self.stack.eval_top()),
                            },
                        ),
                    };
                    control
                }
                InstructionKind::Return => VMControl::Return(self.stack.pop().into_eval()),
                InstructionKind::Loop {
                    ref body,
                    loop_kind,
                } => {
                    let control = match mode {
                        Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
                        Mode::TrackMutation => {
                            self.exec_loop_tracking_mutation(loop_kind, body).into()
                        }
                        Mode::TrackHistory => {
                            self.save_snapshot();
                            let control = self.exec_loop_tracking_mutation(loop_kind, body).into();
                            let (snapshot, mutations) = self.collect_block_mutations();
                            self.history.write(
                                ins,
                                HistoryEntry::loop_entry(
                                    loop_kind,
                                    &control,
                                    snapshot,
                                    body.clone(),
                                    mutations,
                                ),
                            );
                            control
                        }
                    };
                    control
                }
                InstructionKind::BreakIfFalse => {
                    let control = if !self.stack.pop().primitive().to_bool() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    control
                }
                InstructionKind::FieldAccessCompiled {
                    linkage: field_access_fp,
                } => todo!(),
                InstructionKind::FieldAccessInterpreted {
                    field_idx,
                    field_access_contract,
                } => {
                    let this = self.stack.pop();
                    self.stack
                        .push(this.field(field_idx as usize, field_access_contract));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                output: Ok(self.stack.eval_top()),
                            },
                        ),
                    };
                    VMControl::None
                }
                InstructionKind::Assert => {
                    let is_condition_satisfied = self.stack.pop().primitive().to_bool();
                    if !is_condition_satisfied {
                        todo!()
                    } else {
                        VMControl::None
                    }
                }
                InstructionKind::Break => {
                    if mode == Mode::TrackHistory {
                        self.history.write(ins, HistoryEntry::Break)
                    }
                    VMControl::Break
                }
                InstructionKind::ConditionFlow { ref branches } => {
                    self.exec_condition_flow(sheet, ins, branches, mode)
                }
                InstructionKind::PatternMatch { ref branches } => {
                    self.exec_pattern_matching(sheet, ins, branches, mode)
                }
                InstructionKind::NewXml {
                    name,
                    ref props,
                    n_child_expr,
                } => {
                    if n_child_expr > 0 {
                        todo!()
                    }
                    let arguments = self.stack.drain(props.len().try_into().unwrap());
                    let xml_value = XmlValue {
                        name: name.to_string(),
                        props: zip(
                            props.iter().map(|ident| *ident),
                            arguments
                                .into_iter()
                                .map(|argument| argument.to_json_value()),
                        )
                        .collect(),
                    };
                    self.stack
                        .push(StackValue::Owned(OwnedValue::new(xml_value)));
                    match mode {
                        Mode::Fast => (),
                        Mode::TrackMutation => todo!(),
                        Mode::TrackHistory => todo!(),
                    }
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

    pub(crate) fn eval_linkage(&mut self, linkage: Linkage) -> EvalResult<'eval> {
        let mut inputs = self.stack.drain(linkage.nargs);
        should_eq!(self.stack.len(), 0);
        Ok((linkage.call)(&mut inputs)?.into_eval())
    }
}
