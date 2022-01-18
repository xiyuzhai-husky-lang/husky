use crate::*;

impl<'stack> VirtualStack<'stack> {
    pub fn exec_all(&mut self, instructions: &[Instruction]) -> VirtualStackResult<()> {
        for ins in instructions {
            self.exec(&ins.kind)?
        }
        Ok(())
    }

    pub fn exec(&mut self, ins: &InstructionKind) -> VirtualStackResult<()> {
        match ins {
            InstructionKind::PushVarInput(contract, rel_idx) => {
                self.push(self.var(*rel_idx)?.as_input(*contract)?)
            }
            InstructionKind::PushPrimitive(value) => self.push(value.into()),
            InstructionKind::Call(f, nargs) => self.call(*f, *nargs),
            InstructionKind::PrimitiveOpn(opn) => self.exec_builtin_arithmetic(*opn),
            InstructionKind::CallInterpret(instructions) => self.exec_all(&instructions),
        }
    }
}

impl<'stack> VirtualStack<'stack> {
    pub fn push(&mut self, item: VirtualStackValue<'stack>) -> VirtualStackResult<()> {
        self.values.push(item);
        if self.len() >= self.stack_size {
            todo!()
        }
        Ok(())
    }

    pub fn finish(&mut self) -> VirtualStackResult<VirtualStackValue> {
        if self.len() != 1 {
            todo!()
        }
        Ok(self.values.pop().unwrap())
    }
}

impl<'stack> VirtualStack<'stack> {
    fn call(
        &mut self,
        f: fn(&mut Self) -> VirtualStackResult<()>,
        nargs: u16,
    ) -> VirtualStackResult<()> {
        let save = self.current_frame_start;
        // take nargs
        self.current_frame_start = (self.len() - (nargs as usize)) as u16;
        f(self)?;
        // keep only the first item in the new frame
        self.rollback(save)
    }

    fn rollback(&mut self, save: u16) -> VirtualStackResult<()> {
        self.current_frame_start = save;
        self.shrink_to(save as usize + 1)
    }

    fn shrink_to(&mut self, new_len: usize) -> VirtualStackResult<()> {
        if new_len < self.current_frame_start as usize + 1 {
            todo!()
        }
        if new_len > self.len() {
            todo!()
        }
        self.values.truncate(new_len as usize);
        Ok(())
    }

    fn shrink_by(&mut self, decr: u16) -> VirtualStackResult<()> {
        self.shrink_to(self.len() - (decr as usize))
    }

    fn top_i32(&self) -> VirtualStackResult<i32> {
        self.top(0)?.as_i32()
    }

    fn top_f32(&self) -> VirtualStackResult<f32> {
        self.top(0)?.as_f32()
    }

    fn top_u32(&self) -> VirtualStackResult<u32> {
        self.top(0)?.as_u32()
    }

    fn top_bool(&self) -> VirtualStackResult<bool> {
        self.top(0)?.as_bool()
    }

    fn exec_builtin_arithmetic(&mut self, opn: PrimitiveOpn) -> VirtualStackResult<()> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }
        match opn {
            PrimitiveOpn::Add => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_op((a + self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_op((a + self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::AddAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_assign(idx, (a + self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_assign(idx, (a + self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::And => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.binary_op((a && self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::AndAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.binary_assign(idx, (a && self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitAnd => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_op((a & self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitAndAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_assign(idx, (a & self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitOr => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_op((a | self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitOrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_assign(idx, (a | self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitXor => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_op((a ^ self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::BitXorAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_assign(idx, (a ^ self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Div => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_op((a / self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_op((a / self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::DivAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_assign(idx, (a / self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_assign(idx, (a / self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Mul => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_op((a * self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_op((a * self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::MulAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_assign(idx, (a * self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_assign(idx, (a * self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Or => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.binary_op((a || self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::OrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.binary_assign(idx, (a || self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::RemEuclid => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_op(a.rem_euclid(self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_op(a.rem_euclid(self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::RemEuclidAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => {
                    self.binary_assign(idx, a.rem_euclid(self.top_i32()?).into())
                }
                PrimitiveValue::F32(a) => {
                    self.binary_assign(idx, a.rem_euclid(self.top_f32()?).into())
                }
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Shl => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_op((a << self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::ShlAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_assign(idx, (a << self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Shr => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_op((a >> self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::ShrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.binary_assign(idx, (a >> self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::Sub => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_op((a - self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_op((a - self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            PrimitiveOpn::SubAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.binary_assign(idx, (a - self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.binary_assign(idx, (a - self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
        }
    }

    fn binary_op(&mut self, value: PrimitiveValue) -> VirtualStackResult<()> {
        *self.top_mut(1)? = value.into();
        self.shrink_by(1)
    }

    fn binary_assign(&mut self, idx: u16, value: PrimitiveValue) -> VirtualStackResult<()> {
        *self.var_mut(idx)? = value.into();
        self.shrink_by(1)
    }

    fn top(&self, rel_idx: u16) -> VirtualStackResult<&VirtualStackValue<'stack>> {
        let rel_idx = rel_idx as usize;
        if self.len() - 1 - rel_idx < self.current_frame_start as usize {
            todo!()
        }
        Ok(&self.values[(self.len() - 1 - rel_idx) as usize])
    }

    fn top_mut(&mut self, rel_idx: u16) -> VirtualStackResult<&mut VirtualStackValue<'stack>> {
        let rel_idx = rel_idx as usize;
        if self.len() - 1 - rel_idx < self.current_frame_start as usize {
            todo!()
        }
        let len = self.len();
        Ok(&mut self.values[(len - 1 - rel_idx) as usize])
    }
}
