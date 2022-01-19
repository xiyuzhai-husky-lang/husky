mod any;
mod contract;
mod error;
mod impl_exec;
mod instruction;
mod primitive;
#[cfg(test)]
mod tests;
mod value;

pub use any::{Any, HasRef};
pub use contract::InputContract;
pub use error::{VirtualStackError, VirtualStackResult};
pub use instruction::*;
pub use primitive::*;
pub use value::VirtualStackValue;

pub struct VirtualStack<'stack> {
    values: Vec<VirtualStackValue<'stack>>,
    current_frame_start: usize,
    stack_size: usize,
}

impl<'stack> std::fmt::Debug for VirtualStack<'stack> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nVirtualStack {\n")?;
        f.write_str("    items: [\n")?;
        for item in &self.values[0..self.len()] {
            f.write_fmt(format_args!("        {:?}\n", item))?;
        }
        f.write_str("    ],\n")?;
        f.write_str("}\n")
    }
}

impl<'stack> VirtualStack<'stack> {
    pub fn new() -> Self {
        Self::new_with_size(1 << 16)
    }

    pub fn new_with_size(stack_size: usize) -> Self {
        assert!(stack_size <= u16::MAX as usize + 1);
        let mut values = vec![];
        values.reserve(stack_size as usize);
        Self {
            values,
            current_frame_start: 0,
            stack_size,
        }
    }
}

impl<'stack> VirtualStack<'stack> {
    pub fn var(&self, rel_idx: usize) -> VirtualStackResult<&VirtualStackValue<'stack>> {
        if (self.current_frame_start + rel_idx) as usize >= self.len() {
            todo!()
        }
        Ok(&self.values[self.current_frame_start + rel_idx])
    }

    pub fn var_mut(
        &mut self,
        rel_idx: usize,
    ) -> VirtualStackResult<&mut VirtualStackValue<'stack>> {
        if self.current_frame_start + rel_idx >= self.len() {
            todo!()
        }
        Ok(&mut self.values[(self.current_frame_start + rel_idx) as usize])
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[derive(Clone)]
pub struct Compiled(pub fn(&mut VirtualStack) -> VirtualStackResult<()>);

impl std::fmt::Debug for Compiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("Compiled(")?;
        (self.0 as usize).fmt(f)?;
        f.write_str(")")
    }
}

impl PartialEq for Compiled {
    fn eq(&self, other: &Self) -> bool {
        (self.0 as usize) == (other.0 as usize)
    }
}

impl Eq for Compiled {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSignal {
    Normal,
    Return,
    Break,
}
