mod exec_call;
mod exec_condition_flow;
mod exec_feature_eval;
mod exec_interpret_call;
mod exec_loop;
mod exec_pattern_match;

use crate::*;

use husky_check_utils::should_eq;

impl<'temp> Interpreter<'temp> {
    pub(crate) fn exec_all(&mut self, _sheet: &Instructions, _mode: Mode) -> VMControl {
        todo!()
        // for ins in &sheet.instructions {
        //     if self.vm_config.verbose {
        //         println!(
        //             "{} {}:{}",
        //             "exec".red(),
        //             get_relative_path(&ins.src.source())
        //                 .as_os_str()
        //                 .to_str()
        //                 .unwrap()
        //                 .green(),
        //             format!(
        //                 "{:?} .. {:?}",
        //                 ins.src.text_range().start,
        //                 ins.src.text_range().end
        //             )
        //             .bright_yellow(),
        //         )
        //     }
        //     let control = match ins.variant {
        //         InstructionData::PushVariable {
        //             binding,
        //             stack_idx,
        //             range,
        //             ty,
        //             varname,
        //             explicit,
        //         } => {
        //             let value = self.stack.push_variable(stack_idx, binding);
        //             match mode {
        //                 Mode::Fast => (),
        //                 Mode::TrackMutation => match binding {
        //                     Binding::TempMut => self.record_mutation(
        //                         stack_idx,
        //                         varname,
        //                         ins.src.source(),
        //                         range,
        //                         ty,
        //                     ),
        //                     _ => (),
        //                 },
        //                 Mode::TrackHistory => {
        //                     if explicit {
        //                         self.history.write(
        //                             ins,
        //                             HistoryEntry::PureExpr {
        //                                 result: Ok(value.snapshot()),
        //                                 ty,
        //                             },
        //                         )
        //                     }
        //                 }
        //             }
        //             VMControl::None
        //         }
        //         InstructionData::PushEntityFp {
        //             opt_linkage, ty, ..
        //         } => {
        //             self.stack.push(
        //                 __VirtualFunction::ThickFp(opt_linkage.unwrap().transfer()).to_register(),
        //             );
        //             if mode == Mode::TrackHistory {
        //                 self.history.write(
        //                     ins,
        //                     HistoryEntry::PureExpr {
        //                         result: Ok(self.stack.eval_top()),
        //                         ty,
        //                     },
        //                 )
        //             }
        //             VMControl::None
        //         }
        //         InstructionData::PushLiteralValue {
        //             ref value,
        //             ty,
        //             explicit,
        //         } => {
        //             self.stack.push(value.clone());
        //             match mode {
        //                 Mode::Fast | Mode::TrackMutation => (),
        //                 Mode::TrackHistory => {
        //                     if explicit {
        //                         self.history.write(
        //                             ins,
        //                             HistoryEntry::PureExpr {
        //                                 result: Ok(self.stack.eval_top()),
        //                                 ty,
        //                             },
        //                         )
        //                     }
        //                 }
        //             }
        //             VMControl::None
        //         }
        //         InstructionData::CallRoutine {
        //             resolved_linkage,
        //             nargs,
        //             return_ty,
        //             discard,
        //         } => {
        //             // p!(ins.src.file(), ins.src.text_range());
        //             if self.stack.len() > 0 {
        //                 assert_ne!(
        //                     self.stack.eval_top().vtable as *const _,
        //                     &__VOID_VTABLE as *const _
        //                 );
        //             }
        //             let control = self
        //                 .call_specific_routine(resolved_linkage, nargs, discard)
        //                 .into();
        //             match mode {
        //                 Mode::Fast | Mode::TrackMutation => (),
        //                 Mode::TrackHistory => self.history.write(
        //                     ins,
        //                     HistoryEntry::PureExpr {
        //                         result: match control {
        //                             VMControl::Err(ref e) => Err(e.clone().into()),
        //                             _ => Ok(self.stack.eval_top()),
        //                         },
        //                         ty: return_ty,
        //                     },
        //                 ),
        //             }
        //             control
        //         }
        //         InstructionData::CallInterpreted {
        //             routine_uid,
        //             nargs, // including this
        //             return_ty,
        //             discard,
        //         } => {
        //             let instruction_sheet =
        //                 self.db.item_opt_instruction_sheet_by_uid(routine_uid);
        //             let result = self.call_interpreted(&instruction_sheet.unwrap(), nargs, discard);
        //             match mode {
        //                 Mode::Fast | Mode::TrackMutation => (),
        //                 Mode::TrackHistory => {
        //                     let result = match result {
        //                         Ok(()) => Ok(self.stack.eval_top()),
        //                         Err(ref e) => Err(e.clone()),
        //                     };
        //                     self.history.write(
        //                         ins,
        //                         HistoryEntry::PureExpr {
        //                             result,
        //                             ty: return_ty,
        //                         },
        //                     );
        //                 }
        //             };
        //             result.into()
        //         }
        //         InstructionData::NewVirtualStruct { ty, ref fields } => {
        //             self.push_new_virtual_struct(ty, fields);
        //             match mode {
        //                 Mode::Fast | Mode::TrackMutation => (),
        //                 Mode::TrackHistory => {
        //                     let output = self.stack.eval_top();
        //                     self.history.write(
        //                         ins,
        //                         HistoryEntry::PureExpr {
        //                             result: Ok(output),
        //                             ty,
        //                         },
        //                     )
        //                 }
        //             }
        //             VMControl::None
        //         }
        //         InstructionData::Return { .. } => {
        //             let return_value = self.stack.pop();
        //             VMControl::Return(return_value)
        //         }
        //         InstructionData::Loop {
        //             ref body,
        //             loop_kind,
        //         } => {
        //             let control = match mode {
        //                 Mode::Fast => self.exec_loop_fast(loop_kind, body).into(),
        //                 Mode::TrackMutation => {
        //                     self.exec_loop_tracking_mutation(loop_kind, body).into()
        //                 }
        //                 Mode::TrackHistory => {
        //                     self.save_snapshot("Loop".to_string());
        //                     let control = self.exec_loop_tracking_mutation(loop_kind, body).into();
        //                     let (snapshot, mutations) = self.collect_block_mutations();
        //                     self.history.write(
        //                         ins,
        //                         HistoryEntry::loop_entry(
        //                             loop_kind,
        //                             &control,
        //                             snapshot,
        //                             body.clone(),
        //                             mutations,
        //                         ),
        //                     );
        //                     control
        //                 }
        //             };
        //             control
        //         }
        //         InstructionData::BreakIfFalse => {
        //             let control = if !self.stack.pop().to_bool() {
        //                 VMControl::Break
        //             } else {
        //                 VMControl::None
        //             };
        //             control
        //         }
        //         InstructionData::VirtualStructField {
        //             field_idx,
        //             field_binding,
        //             field_ty,
        //         } => {
        //             let this = self.stack.pop();
        //             self.stack
        //                 .push(this.virtual_struct_field(field_idx, field_binding));
        //             match mode {
        //                 Mode::Fast | Mode::TrackMutation => (),
        //                 Mode::TrackHistory => self.history.write(
        //                     ins,
        //                     HistoryEntry::PureExpr {
        //                         result: Ok(self.stack.eval_top()),
        //                         ty: field_ty,
        //                     },
        //                 ),
        //             };
        //             VMControl::None
        //         }
        //         InstructionData::Assert => {
        //             let is_condition_satisfied = self.stack.pop().to_bool();
        //             if !is_condition_satisfied {
        //                 VMControl::Err(__VMError::new_normal(format!("assert failure")))
        //             } else {
        //                 VMControl::None
        //             }
        //         }
        //         InstructionData::Require => {
        //             let is_condition_satisfied = self.stack.pop().to_bool();
        //             if !is_condition_satisfied {
        //                 VMControl::Return(__RegularValue::none(0))
        //             } else {
        //                 VMControl::None
        //             }
        //         }
        //         InstructionData::Break => {
        //             if mode == Mode::TrackHistory {
        //                 self.history.write(ins, HistoryEntry::Break)
        //             }
        //             VMControl::Break
        //         }
        //         InstructionData::ConditionFlow { ref branches } => {
        //             self.exec_condition_flow(ins, branches, mode)
        //         }
        //         InstructionData::PatternMatch { ref branches } => {
        //             self.exec_pattern_matching(ins, branches, mode)
        //         }
        //         InstructionData::EntityFeature { feature_uid, ty } => {
        //             self.exec_feature_eval(feature_uid, mode, ins, ty).into()
        //         }
        //         InstructionData::WrapInSome { number_of_somes: _ } => todo!(),
        //     };
        //     match control {
        //         VMControl::None => (),
        //         VMControl::Break | VMControl::Return(_) | VMControl::Err(_) => return control,
        //     }
        // }
        // VMControl::None
    }

    pub(crate) fn eval_linkage(
        &mut self,
        linkage: __LinkageGroup,
        nargs: u8,
    ) -> __VMResult<__RegularValue> {
        match linkage {
            __LinkageGroup::Member { .. } => todo!(),
            __LinkageGroup::Transfer(linkage) => {
                let arguments = self.stack.drain(nargs).collect::<Vec<_>>();
                should_eq!(self.stack.len(), 0);
                linkage.call_catch_unwind(self.opt_ctx, arguments)
            }
            __LinkageGroup::Model(_) => todo!(),
        }
    }
}
