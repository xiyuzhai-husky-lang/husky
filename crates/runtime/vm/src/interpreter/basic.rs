// mod impl_exec;
#[cfg(test)]
mod tests;

use crate::*;
pub struct BasicInterpreter<'stack, 'eval: 'stack> {
    values: Vec<VMValue<'stack, 'eval>>,
}

impl<'stack, 'eval: 'stack> std::fmt::Debug for BasicInterpreter<'stack, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn new(values: Vec<VMValue<'stack, 'eval>>) -> Self {
        Self { values }
    }
}

impl<'stack, 'eval: 'stack> Interpreter<'stack, 'eval> for BasicInterpreter<'stack, 'eval> {
    fn init(&mut self, init_kind: InitKind) -> VMResult<()> {
        match init_kind {
            InitKind::Let | InitKind::Decl => Ok(()),
            InitKind::Var => match self.values.last().unwrap() {
                VMValue::Primitive(_) | VMValue::Boxed(_) => Ok(()),
                VMValue::MutRef(_)
                | VMValue::Volatile(_)
                | VMValue::GlobalRef(_)
                | VMValue::Ref(_) => todo!(),
            },
        }
    }

    fn var(&self, rel_idx: usize) -> VMResult<&VMValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&self.values[rel_idx])
    }

    fn var_mut(&mut self, rel_idx: usize) -> VMResult<&mut VMValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&mut self.values[rel_idx])
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn push(&mut self, value: VMValue<'stack, 'eval>) {
        self.values.push(value)
    }

    fn pop(&mut self) -> VMResult<VMValue<'stack, 'eval>> {
        self.values.pop().ok_or(VMError::CannotPop)
    }

    fn drain(&mut self, new_len: usize) -> Vec<VMValue<'stack, 'eval>> {
        self.values.drain(new_len..).collect()
    }
}
