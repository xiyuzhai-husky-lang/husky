mod impl_exec;
#[cfg(test)]
mod tests;

use crate::*;
pub struct VMStack<'stack, 'eval: 'stack> {
    values: Vec<StackValue<'stack, 'eval>>,
}

impl<'stack, 'eval: 'stack> std::fmt::Debug for VMStack<'stack, 'eval> {
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

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub fn new(values: Vec<StackValue<'stack, 'eval>>) -> Self {
        Self { values }
    }
}

impl<'stack, 'eval: 'stack> VMStack<'stack, 'eval> {
    pub fn var(&self, rel_idx: usize) -> VMResult<&StackValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&self.values[rel_idx])
    }

    pub fn var_mut(&mut self, rel_idx: usize) -> VMResult<&mut StackValue<'stack, 'eval>> {
        if rel_idx >= self.len() {
            todo!()
        }
        Ok(&mut self.values[rel_idx])
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
