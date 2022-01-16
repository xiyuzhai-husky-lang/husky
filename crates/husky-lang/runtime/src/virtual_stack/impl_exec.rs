use crate::*;

use super::*;

impl<'stack> VirtualStack<'stack> {
    pub(super) fn execute_all(&mut self, instructions: &[Instruction]) -> RuntimeResult<()> {
        for ins in instructions {
            self.execute(ins)?
        }
        Ok(())
    }

    pub(super) fn execute(&mut self, ins: &Instruction) -> RuntimeResult<()> {
        match ins {
            Instruction::PushVarInput(contract, rel_idx) => {
                self.push(self.var(*rel_idx)?.as_input(*contract)?)
            }
            Instruction::PushPrimitive(value) => self.push(value.into()),
            Instruction::Call(f, nargs) => self.call(*f, *nargs),
            Instruction::BuiltinArithmetic(opn) => self.exec_builtin_arithmetic(*opn),
        }
    }
}

impl<'stack> VirtualStack<'stack> {
    fn push(&mut self, item: VirtualStackValue<'stack>) -> RuntimeResult<()> {
        self.items[self.len as usize] = item;
        self.len += 1;
        if self.len as usize >= VIRTUAL_STACK_SIZE {
            todo!()
        }
        Ok(())
    }
}

impl<'stack> VirtualStack<'stack> {
    fn call(&mut self, f: fn(&mut Self) -> RuntimeResult<()>, nargs: u16) -> RuntimeResult<()> {
        let save = self.current_frame_start;
        // take nargs
        self.current_frame_start = self.len - nargs;
        f(self)?;
        // keep only the first item in the new frame
        self.rollback(save)
    }

    fn rollback(&mut self, save: u16) -> RuntimeResult<()> {
        self.current_frame_start = save;
        self.shrink_to(save + 1)
    }

    fn shrink_to(&mut self, new_len: u16) -> RuntimeResult<()> {
        if new_len < self.current_frame_start + 1 {
            todo!()
        }
        for i in new_len..self.len {
            // ensure local objects out of scope are deleted
            self.items[i as usize] = VirtualStackValue::Undefined
        }
        self.len = new_len;
        Ok(())
    }

    fn shrink_by(&mut self, decr: u16) -> RuntimeResult<()> {
        self.shrink_to(self.len - decr)
    }

    fn top_i32(&self) -> RuntimeResult<i32> {
        self.top(0)?.as_i32()
    }

    fn top_f32(&self) -> RuntimeResult<f32> {
        self.top(0)?.as_f32()
    }

    fn top_u32(&self) -> RuntimeResult<u32> {
        self.top(0)?.as_u32()
    }

    fn top_bool(&self) -> RuntimeResult<bool> {
        self.top(0)?.as_bool()
    }

    fn exec_builtin_arithmetic(&mut self, opn: BuiltinArithmeticOpn) -> RuntimeResult<()> {
        macro_rules! no_such_opn {
            () => {{
                todo!()
            }};
        }
        match opn {
            BuiltinArithmeticOpn::Add => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_op((a + self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_op((a + self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::AddAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_assign(idx, (a + self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_assign(idx, (a + self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::And => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.bi_op((a && self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::AndAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.bi_assign(idx, (a && self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitAnd => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_op((a & self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitAndAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_assign(idx, (a & self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitOr => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_op((a | self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitOrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_assign(idx, (a | self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitXor => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_op((a ^ self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::BitXorAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_assign(idx, (a ^ self.top_u32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Div => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_op((a / self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_op((a / self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::DivAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_assign(idx, (a / self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_assign(idx, (a / self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Mul => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_op((a * self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_op((a * self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::MulAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_assign(idx, (a * self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_assign(idx, (a * self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Or => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.bi_op((a || self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::OrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::Bool(a) => self.bi_assign(idx, (a || self.top_bool()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::RemEuclid => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_op((a.rem_euclid(self.top_i32()?).into())),
                PrimitiveValue::F32(a) => self.bi_op((a.rem_euclid(self.top_f32()?).into())),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::RemEuclidAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => {
                    self.bi_assign(idx, (a.rem_euclid(self.top_i32()?).into()))
                }
                PrimitiveValue::F32(a) => {
                    self.bi_assign(idx, (a.rem_euclid(self.top_f32()?).into()))
                }
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Shl => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_op((a << self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::ShlAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_assign(idx, (a << self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Shr => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_op((a >> self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::ShrAssign(idx) => match self.top(1)?.as_primitive()? {
                PrimitiveValue::B32(a) => self.bi_assign(idx, (a >> self.top_i32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::Sub => match self.top(1)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_op((a - self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_op((a - self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
            BuiltinArithmeticOpn::SubAssign(idx) => match self.var(idx)?.as_primitive()? {
                PrimitiveValue::I32(a) => self.bi_assign(idx, (a - self.top_i32()?).into()),
                PrimitiveValue::F32(a) => self.bi_assign(idx, (a - self.top_f32()?).into()),
                _ => no_such_opn!(),
            },
        }
    }

    fn bi_op(&mut self, value: PrimitiveValue) -> RuntimeResult<()> {
        *self.top_mut(1)? = value.into();
        self.shrink_by(1)
    }

    fn bi_assign(&mut self, idx: u16, value: PrimitiveValue) -> RuntimeResult<()> {
        *self.var_mut(idx)? = value.into();
        self.shrink_by(1)
    }

    fn top(&self, rel_idx: u16) -> RuntimeResult<&VirtualStackValue<'stack>> {
        if self.len - 1 - rel_idx < self.current_frame_start {
            todo!()
        }
        Ok(&self.items[(self.len - 1 - rel_idx) as usize])
    }

    fn top_mut(&mut self, rel_idx: u16) -> RuntimeResult<&mut VirtualStackValue<'stack>> {
        if self.len - 1 - rel_idx < self.current_frame_start {
            todo!()
        }
        Ok(&mut self.items[(self.len - 1 - rel_idx) as usize])
    }
}
