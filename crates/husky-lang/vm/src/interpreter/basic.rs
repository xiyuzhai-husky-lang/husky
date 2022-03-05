// mod impl_exec;
#[cfg(test)]
mod tests;

use crate::*;
pub struct BasicInterpreter<'stack, 'eval: 'stack> {
    values: Vec<StackValue<'stack, 'eval>>,
}

impl<'stack, 'eval: 'stack> std::fmt::Debug for BasicInterpreter<'stack, 'eval> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nInterpretStack {\n")?;
        f.write_str("    items: [\n")?;
        for item in &self.values[0..self.len()] {
            f.write_fmt(format_args!("        {:?}\n", item))?;
        }
        f.write_str("    ],\n")?;
        f.write_str("}\n")
    }
}

impl<'stack, 'eval: 'stack> BasicInterpreter<'stack, 'eval> {
    pub fn new(values: Vec<StackValue<'stack, 'eval>>) -> Self {
        Self { values }
    }
}

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> for BasicInterpreter<'stack, 'eval> {
    fn init(&mut self, init_kind: InitKind) -> VMResult<()> {
        match init_kind {
            InitKind::Let | InitKind::Decl => Ok(()),
            InitKind::Var => match self.values.last().unwrap() {
                StackValue::Primitive(_) | StackValue::Boxed(_) => Ok(()),
                StackValue::MutRef(_)
                | StackValue::Volatile(_)
                | StackValue::GlobalRef(_)
                | StackValue::Ref(_) => todo!(),
            },
        }
    }

    fn var(&self, rel_idx: usize) -> VMResult<&StackValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&self.values[rel_idx])
    }

    fn var_mut(&mut self, rel_idx: usize) -> VMResult<&mut StackValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&mut self.values[rel_idx])
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn push(&mut self, value: StackValue<'stack, 'eval>) {
        self.values.push(value)
    }

    fn pop(&mut self) -> VMResult<StackValue<'stack, 'eval>> {
        self.values.pop().ok_or(VMError::CannotPop)
    }

    fn drain(&mut self, new_len: usize) -> Vec<StackValue<'stack, 'eval>> {
        self.values.drain(new_len..).collect()
    }
}
