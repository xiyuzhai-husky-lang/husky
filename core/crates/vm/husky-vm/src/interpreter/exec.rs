mod exec_call;
mod exec_condition_flow;
mod exec_feature_eval;
mod exec_interpret_call;
mod exec_loop;
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
                    explicit,
                } => {
                    let value = self.stack.push_variable(stack_idx, binding);
                    match mode {
                        Mode::Fast => (),
                        Mode::TrackMutation => match binding {
                            Binding::TempMut => {
                                self.record_mutation(stack_idx, varname, ins.src.file(), range, ty)
                            }
                            _ => (),
                        },
                        Mode::TrackHistory => {
                            if explicit {
                                self.history.write(
                                    ins,
                                    HistoryEntry::PureExpr {
                                        result: Ok(value.snapshot()),
                                        ty,
                                    },
                                )
                            }
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::PushEntityFp {
                    opt_linkage, ty, ..
                } => {
                    self.stack
                        .push(__VirtualFunction::Fp(opt_linkage.unwrap().transfer()).to_register());
                    if mode == Mode::TrackHistory {
                        self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                                ty,
                            },
                        )
                    }
                    VMControl::None
                }
                InstructionVariant::PushValue {
                    ref value,
                    ty,
                    explicit,
                } => {
                    self.stack.push(unsafe { value.verbatim_copy() });
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            if explicit {
                                self.history.write(
                                    ins,
                                    HistoryEntry::PureExpr {
                                        result: Ok(self.stack.eval_top()),
                                        ty,
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
                                ty: entity_kind.route.parent(),
                            },
                        ),
                    }
                    VMControl::None
                }
                InstructionVariant::CallRoutine {
                    linkage_fp,
                    nargs,
                    output_ty,
                    discard,
                } => {
                    // p!(ins.src.file(), ins.src.text_range());
                    if self.stack.len() > 0 {
                        assert_ne!(self.stack.eval_top().vtable as *const _, unsafe {
                            &__VOID_VTABLE as *const _
                        });
                    }
                    let control = self
                        .call_specific_routine(linkage_fp, nargs, output_ty, discard)
                        .into();
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: match control {
                                    VMControl::Err(ref e) => Err(e.clone().into()),
                                    _ => Ok(self.stack.eval_top()),
                                },
                                ty: output_ty,
                            },
                        ),
                    }
                    control
                }
                InstructionVariant::CallInterpreted {
                    routine_uid,
                    nargs, // including this
                    has_this,
                    output_ty,
                    discard,
                } => {
                    let instruction_sheet =
                        self.db.entity_opt_instruction_sheet_by_uid(routine_uid);
                    let result = self.call_interpreted(
                        &instruction_sheet.unwrap(),
                        nargs,
                        has_this,
                        discard,
                    );
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => {
                            let result = match result {
                                Ok(()) => Ok(self.stack.eval_top()),
                                Err(ref e) => Err(e.clone()),
                            };
                            self.history.write(
                                ins,
                                HistoryEntry::PureExpr {
                                    result,
                                    ty: output_ty,
                                },
                            );
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
                            self.history.write(
                                ins,
                                HistoryEntry::PureExpr {
                                    result: Ok(output),
                                    ty,
                                },
                            )
                        }
                    }
                    VMControl::None
                }
                InstructionVariant::Return { output_ty } => {
                    let return_value = self.stack.pop();
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
                    let control = if !self.stack.pop().to_bool() {
                        VMControl::Break
                    } else {
                        VMControl::None
                    };
                    control
                }
                InstructionVariant::VirtualStructField {
                    field_idx,
                    field_binding,
                    field_ty,
                } => {
                    let this = self.stack.pop();
                    self.stack
                        .push(this.virtual_struct_field(field_idx, field_binding));
                    // this.field(field_idx as usize, field_binding));
                    match mode {
                        Mode::Fast | Mode::TrackMutation => (),
                        Mode::TrackHistory => self.history.write(
                            ins,
                            HistoryEntry::PureExpr {
                                result: Ok(self.stack.eval_top()),
                                ty: field_ty,
                            },
                        ),
                    };
                    VMControl::None
                }
                InstructionVariant::Assert => {
                    let is_condition_satisfied = self.stack.pop().to_bool();
                    if !is_condition_satisfied {
                        VMControl::Err(vm_error!(format!("assert failure")))
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
                InstructionVariant::EntityFeature { feature_uid, ty } => {
                    self.exec_feature_eval(feature_uid, mode, ins, ty).into()
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
                let arguments = self.stack.drain(nargs).collect::<Vec<_>>();
                should_eq!(self.stack.len(), 0);
                linkage.call_catch_unwind(self.opt_ctx, arguments)
            }
            __Linkage::Model(_) => todo!(),
        }
    }
}
