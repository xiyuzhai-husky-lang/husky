use crate::*;

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> {
    pub(super) fn exec_loop_fast(
        &mut self,
        loop_kind: VMLoopKind,
        body: &InstructionSheet,
    ) -> VMControl<'eval> {
        self.exec_loop(loop_kind, body, |_| (), |_, _, _| ()).into()
    }

    pub(crate) fn exec_loop_debug(
        &mut self,
        loop_kind: VMLoopKind,
        body: &InstructionSheet,
    ) -> VMControl<'eval> {
        self.exec_loop(
            loop_kind,
            body,
            |interpreter| interpreter.take_snapshot(),
            |interpreter, frame_var_value, control| {
                let (snapshot, changes) = interpreter.take_changes();
                interpreter.frames.push(LoopFrameSnapshot {
                    stack: snapshot,
                    changes,
                    frame_var_value,
                    control: control.snapshot(),
                    kind: loop_kind.into(),
                });
            },
        )
        .into()
    }

    pub(super) fn exec_loop(
        &mut self,
        loop_kind: VMLoopKind,
        body: &InstructionSheet,
        exec_before_each_frame: impl Fn(&mut Self),
        exec_after_each_frame: impl Fn(&mut Self, i32, &VMControl<'eval>),
    ) -> VMResult<VMControl<'eval>> {
        match loop_kind {
            VMLoopKind::For {
                initial_boundary_kind,
                final_boundary_kind,
                step,
                ..
            } => {
                let initial_bound_shifted = {
                    let initial_bound = self.stack.top_second().as_primitive()?.as_i32()?;
                    match initial_boundary_kind {
                        BoundaryKind::UpperOpen => initial_bound - 1,
                        BoundaryKind::UpperClosed => initial_bound,
                        BoundaryKind::LowerOpen => initial_bound + 1,
                        BoundaryKind::LowerClosed => initial_bound,
                    }
                };
                let final_bound_shifted = {
                    let final_bound = self.stack.top().as_primitive()?.as_i32()?;
                    match final_boundary_kind {
                        BoundaryKind::UpperOpen => final_bound - 1,
                        BoundaryKind::UpperClosed => final_bound,
                        BoundaryKind::LowerOpen => final_bound + 1,
                        BoundaryKind::LowerClosed => final_bound,
                    }
                };
                let n = step.n(initial_bound_shifted, final_bound_shifted);
                for i in 0..n {
                    let frame_var = step.frame_var(initial_bound_shifted, i);
                    self.stack.push(StackValue::Primitive(frame_var.into()));
                    exec_before_each_frame(self);
                    let control = self.exec_all(&body.instructions, Mode::Fast);
                    exec_after_each_frame(self, frame_var, &control);
                    self.stack.pop();
                    match control {
                        VMControl::None => (),
                        VMControl::Return(value) => return Ok(VMControl::Return(value)),
                        VMControl::Break => return Ok(VMControl::None),
                        VMControl::Err(_) => todo!(),
                    }
                }
                Ok(VMControl::None)
            }
            VMLoopKind::ForExt {
                frame_varidx,
                final_boundary_kind,
                step,
                ..
            } => {
                let initial_value = self.stack.value(frame_varidx).as_primitive()?.as_i32()?;
                let final_bound_shifted = {
                    let final_bound = self.stack.top().as_primitive()?.as_i32()?;
                    match final_boundary_kind {
                        BoundaryKind::UpperOpen => final_bound - 1,
                        BoundaryKind::UpperClosed => final_bound,
                        BoundaryKind::LowerOpen => final_bound + 1,
                        BoundaryKind::LowerClosed => final_bound,
                    }
                };
                let n = step.n(initial_value, final_bound_shifted);
                for _ in 0..n {
                    exec_before_each_frame(self);
                    let control = self.exec_all(&body.instructions, Mode::Fast);
                    exec_after_each_frame(
                        self,
                        self.stack.value(frame_varidx).as_primitive()?.as_i32()?,
                        &control,
                    );
                    match control {
                        VMControl::None => (),
                        VMControl::Return(value) => return Ok(VMControl::Return(value)),
                        VMControl::Break => return Ok(VMControl::None),
                        VMControl::Err(_) => todo!(),
                    }
                    step.update(self.stack.value_mut(frame_varidx));
                }
                Ok(VMControl::None)
            }
            VMLoopKind::Loop => {
                for frame_var in 0..LOOP_LIMIT {
                    exec_before_each_frame(self);
                    let control = self.exec_all(&body.instructions, Mode::Fast);
                    exec_after_each_frame(self, frame_var, &control);
                    match control {
                        VMControl::None => (),
                        VMControl::Return(value) => return Ok(VMControl::Return(value)),
                        VMControl::Break => return Ok(VMControl::None),
                        VMControl::Err(_) => todo!(),
                    }
                }
                err!(format!("infinite loop (loop limit = {})", LOOP_LIMIT))
            }
        }
    }
}

const LOOP_LIMIT: i32 = 50000;
