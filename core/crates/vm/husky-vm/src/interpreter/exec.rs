mod exec_call;
mod exec_condition_flow;
mod exec_interpret_call;
mod exec_loop;
mod exec_opr_opn;
mod exec_pattern_match;

use crate::{history::HistoryEntry, *};
use colored::Colorize;
use husky_check_utils::{should, should_eq};
use husky_entity_route::EntityRouteKind;
use husky_print_utils::{msg_once, p, ps};
use husky_word::RootIdentifier;
use path_utils::get_relative_path;
use std::iter::zip;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(crate) fn exec_all(&mut self, sheet: &InstructionSheet, mode: Mode) -> VMControl<'eval> {
        for ins in &sheet.instructions {
            if self.vm_config.verbose {
                println!(
                    "{} {}:{}",
                    "exec".red(),
                    get_relative_path(&ins.src.file())
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .green(),
                    format!(
                        "{:?} .. {:?}",
                        ins.src.text_range().start,
                        ins.src.text_range().end
                    )
                    .bright_yellow(),
                )
            }
            let control = match ins.variant {
                InstructionVariant::PushVariable {
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
                            Binding::TempRefMut => {
                                self.record_mutation(stack_idx, varname, ins.src.file(), range, ty)
                            }
                            _ => (),
                        },
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(value.__eval__()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionVariant::PushEntityFp { opt_linkage, .. } => {
                    self.stack.push(
                        todo!(), // __TempValue::owned_eval(__CallFormValue { opt_linkage })
                    );
                    if mode == Mode::TrackHistory {
                        self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                            },
                        )
                    }
                    VMControl::None
                }
                InstructionVariant::PushPrimitiveValue { value, explicit } => {
                    self.stack.push(unsafe { value.to_register() });
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            if explicit {
                                self.history.write(
                                    ins,
                                    HistoryEntry::PureExpr {
                                        result: Ok(self.stack.eval_top()),
                                    },
                                )
                            }
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::PushEnumKindLiteral(entity_kind) => {
                    self.stack.push(
                        todo!(), // __TempValue::Copyable(entity_kind.into())
                    );
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionVariant::CallRoutine {
                    linkage_fp: linkage,
                    nargs,
                    output_ty,
                } => {
                    let control = self.call_specific_routine(linkage, nargs, output_ty).into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: match control {
                                    VMControl::Err(ref e) => Err(e.clone().into()),
                                    _ => Ok(self.stack.eval_top()),
                                },
                            },
                        ),
                    }
                    control
                }
                InstructionVariant::OprOpn { opn, .. } => {
                    // sheet.variable_stack.compare_with_vm_stack(&self.stack);
                    self.exec_opr_opn(opn, mode, ins).into()
                }
                InstructionVariant::CallInterpreted {
                    routine_uid: routine,
                    nargs, // including this
                    has_this,
                    output_ty,
                } => {
                    let instruction_sheet = self.db.entity_opt_instruction_sheet_by_uid(routine);
                    let result =
                        self.call_interpreted(&instruction_sheet.unwrap(), nargs, has_this);
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            let result = match result {
                                Ok(()) => Ok(self.stack.eval_top()),
                                Err(ref e) => Err(e.clone()),
                            };
                            self.history.write(ins, HistoryEntry::PureExpr { result })
                        }
                    };
                    result.into()
                }
                InstructionVariant::NewVirtualStruct { ty, ref fields } => {
                    self.push_new_virtual_struct(ty, fields);
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            let output = self.stack.eval_top();
                            should_eq!(output.ty(), ty);
                            self.history
                                .write(ins, HistoryEntry::PureExpr { result: Ok(output) })
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::Return { output_ty } => {
                    let return_value = self.stack.pop().__eval__();
                    msg_once!("ugly");
                    if output_ty.kind
                        != (EntityRouteKind::Root {
                            ident: RootIdentifier::DatasetType,
                        })
                    {
                        should_eq!(output_ty, return_value.ty());
                    }
                    VMControl::Return(return_value)
                }
                InstructionVariant::Loop {
                    ref body,
                    loop_kind,
                } => {
                    let control = match mode {
                        Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
                        Mode::TrackMutation => {
                            self.exec_loop_tracking_mutation(loop_kind, body).into()
                        }
                        Mode::TrackHistory => {
                            self.save_snapshot("Loop".to_string());
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
                InstructionVariant::BreakIfFalse => {
                    let control = if !self.stack.pop().primitive().to_bool() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    control
                }
                InstructionVariant::FieldAccessInterpreted {
                    field_idx,
                    field_binding,
                } => {
                    let this = self.stack.pop();
                    self.stack.push(todo!());
                    // this.field(field_idx as usize, field_binding));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                            },
                        ),
                    };
                    VMControl::None
                }
                InstructionVariant::Assert => {
                    let is_condition_satisfied = self.stack.pop().primitive().to_bool();
                    if !is_condition_satisfied {
                        VMControl::Err(vm_runtime_error!(format!("assert failure")))
                    } else {
                        VMControl::None
                    }
                }
                InstructionVariant::Break => {
                    if mode == Mode::TrackHistory {
                        self.history.write(ins, HistoryEntry::Break)
                    }
                    VMControl::Break
                }
                InstructionVariant::ConditionFlow { ref branches } => {
                    self.exec_condition_flow(sheet, ins, branches, mode)
                }
                InstructionVariant::PatternMatch { ref branches } => {
                    self.exec_pattern_matching(sheet, ins, branches, mode)
                }
                InstructionVariant::EntityFeature { feature_uid } => {
                    p!(ins.src.file(), ins.src.text_range());
                    todo!()
                }
            };
            match control {
                VMControl::None => (),
                VMControl::Break | VMControl::Return(_) | VMControl::Err(_) => return control,
            }
        }
        VMControl::None
    }

    pub(crate) fn eval_linkage(
        &mut self,
        linkage: __Linkage,
        nargs: u8,
        output_ty: EntityRoutePtr,
    ) -> __VMResult<__Register<'eval>> {
        match linkage {
            __Linkage::Member { .. } => todo!(),
            __Linkage::Transfer(linkage) => {
                let mut arguments = self.stack.drain(nargs).collect::<Vec<_>>();
                should_eq!(self.stack.len(), 0);
                linkage.eval(self.opt_ctx, &mut arguments)
            }
            __Linkage::Model(_) => todo!(),
        }
    }
}
