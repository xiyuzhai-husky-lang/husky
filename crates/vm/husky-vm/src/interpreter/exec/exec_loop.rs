use crate::*;

impl<'temp> Interpreter<'temp> {
    pub(super) fn exec_loop_fast(
        &mut self,
        loop_kind: VMLoopKind,
        body: &Instructions,
    ) -> VMControl {
        self.exec_loop(loop_kind, body, |_| (), |_, _, _| (), Mode::Fast)
            .into()
    }

    pub(super) fn exec_loop_tracking_mutation(
        &mut self,
        loop_kind: VMLoopKind,
        body: &Instructions,
    ) -> VMControl {
        self.exec_loop(loop_kind, body, |_| (), |_, _, _| (), Mode::TrackMutation)
            .into()
    }

    pub(crate) fn exec_loop_tracking_frame(
        &mut self,
        loop_kind: VMLoopKind,
        body: &Instructions,
    ) -> VMControl {
        self.exec_loop(
            loop_kind,
            body,
            |interpreter| interpreter.save_snapshot(format!("loop frame")),
            |interpreter, frame_var_value, control| {
                let (snapshot, mutations) = interpreter.collect_block_mutations();
                interpreter.frames.push(LoopFrameData {
                    stack_snapshot: snapshot,
                    mutations,
                    frame_var_value,
                    control: control.snapshot(),
                    frame_kind: loop_kind.into(),
                });
            },
            Mode::TrackMutation,
        )
        .into()
    }

    pub(super) fn exec_loop(
        &mut self,
        loop_kind: VMLoopKind,
        body: &Instructions,
        exec_before_each_frame: impl Fn(&mut Self),
        exec_after_each_frame: impl Fn(&mut Self, i32, &VMControl),
        mode: Mode,
    ) -> __VMResult<VMControl> {
        let control = match loop_kind {
            VMLoopKind::For {
                initial_boundary_kind,
                final_boundary_kind,
                step,
                ..
            } => {
                let final_bound_shifted = {
                    todo!()
                    // let final_bound = self.stack.pop().downcast_i32();
                    // match final_boundary_kind {
                    //     BoundaryKind::UpperOpen => final_bound - 1,
                    //     BoundaryKind::UpperClosed => final_bound,
                    //     BoundaryKind::LowerOpen => final_bound + 1,
                    //     BoundaryKind::LowerClosed => final_bound,
                    // }
                };
                let initial_bound_shifted = {
                    todo!()
                    // let initial_bound = self.stack.pop().downcast_i32();
                    // match initial_boundary_kind {
                    //     BoundaryKind::UpperOpen => initial_bound - 1,
                    //     BoundaryKind::UpperClosed => initial_bound,
                    //     BoundaryKind::LowerOpen => initial_bound + 1,
                    //     BoundaryKind::LowerClosed => initial_bound,
                    // }
                };
                let n = step.n(initial_bound_shifted, final_bound_shifted);
                let mut control = VMControl::None;
                let stack_len = self.stack.len();
                for i in 0..n {
                    let frame_var = step.frame_var(initial_bound_shifted, i);
                    self.stack.push(unsafe {
                        todo!()
                        // frame_var.__to_register()
                    });
                    exec_before_each_frame(self);
                    let frame_control = self.exec_all(body, mode);
                    exec_after_each_frame(self, frame_var, &frame_control);
                    self.stack.truncate(stack_len);
                    match frame_control {
                        VMControl::None => (),
                        VMControl::Return(value) => {
                            control = VMControl::Return(value);
                            break;
                        }
                        VMControl::Break => {
                            control = VMControl::None;
                            break;
                        }
                        VMControl::Err(e) => {
                            control = VMControl::Err(e);
                            break;
                        }
                    }
                }
                Ok(control)
            }
            VMLoopKind::ForExt {
                frame_varidx,
                final_boundary_kind,
                step,
                ..
            } => {
                todo!()
                // let initial_value = self.stack.value(frame_varidx).downcast_i32();
                // let final_bound_shifted = {
                //     let final_bound = self.stack.pop().downcast_i32();
                //     match final_boundary_kind {
                //         BoundaryKind::UpperOpen => final_bound - 1,
                //         BoundaryKind::UpperClosed => final_bound,
                //         BoundaryKind::LowerOpen => final_bound + 1,
                //         BoundaryKind::LowerClosed => final_bound,
                //     }
                // };
                // let n = step.n(initial_value, final_bound_shifted);
                // let mut control = VMControl::None;
                // let stack_len = self.stack.len();
                // for _ in 0..n {
                //     exec_before_each_frame(self);
                //     let frame_control = self.exec_all(body, mode);
                //     exec_after_each_frame(
                //         self,
                //         self.stack.value(frame_varidx).downcast_i32(),
                //         &frame_control,
                //     );
                //     self.stack.truncate(stack_len);
                //     match frame_control {
                //         VMControl::None => (),
                //         VMControl::Return(value) => {
                //             control = VMControl::Return(value);
                //             break;
                //         }
                //         VMControl::Break => {
                //             control = VMControl::None;
                //             break;
                //         }
                //         VMControl::Err(e) => {
                //             control = VMControl::Err(e);
                //             break;
                //         }
                //     }
                //     unsafe {
                //         *self
                //             .stack
                //             .value_mut(frame_varidx)
                //             .downcast_temp_mut::<i32>(&__I32_VTABLE) += step.0
                //     }
                // }
                // Ok(control)
            }
            VMLoopKind::Loop => {
                // ugly, urhh
                let mut control_result = Err(__VMError::new_normal(format!(
                    "infinite loop (loop limit = {})",
                    LOOP_LIMIT
                )));
                let stack_len = self.stack.len();
                for frame_var in 0..LOOP_LIMIT {
                    exec_before_each_frame(self);
                    let frame_control = self.exec_all(body, mode);
                    exec_after_each_frame(self, frame_var, &frame_control);
                    self.stack.truncate(stack_len);
                    match frame_control {
                        VMControl::None => (),
                        VMControl::Return(value) => {
                            control_result = Ok(VMControl::Return(value));
                            break;
                        }
                        VMControl::Break => {
                            control_result = Ok(VMControl::None);
                            break;
                        }
                        VMControl::Err(e) => {
                            control_result = Err(e);
                            break;
                        }
                    }
                }
                control_result
            }
        };
        control
    }
}

const LOOP_LIMIT: i32 = 500;
