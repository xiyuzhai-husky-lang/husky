mod any;
mod error;
mod impl_exec;
mod instruction;
#[cfg(test)]
mod tests;
mod value;

pub use any::Any;
pub use error::{VirtualStackError, VirtualStackResult};
pub use instruction::*;
pub use value::VirtualStackValue;

use syntax_types::*;

pub struct VirtualStack<'stack> {
    items: Box<[VirtualStackValue<'stack>; VIRTUAL_STACK_SIZE]>,
    current_frame_start: u16,
    len: u16,
}

impl<'stack> std::fmt::Debug for VirtualStack<'stack> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nVirtualStack {\n")?;
        f.write_str("    items: [\n")?;
        for item in &self.items[0..self.len()] {
            f.write_fmt(format_args!("        {:?}\n", item))?;
        }
        f.write_str("    ],\n")?;
        f.write_str("}\n")
    }
}

const VIRTUAL_STACK_SIZE: usize = 1 << 16;

impl<'stack> VirtualStack<'stack> {
    pub fn new() -> Self {
        const DEFAULT_VIRTUAL_STACK_ITEM: VirtualStackValue = VirtualStackValue::Undefined;
        VirtualStack {
            items: Box::new([DEFAULT_VIRTUAL_STACK_ITEM; VIRTUAL_STACK_SIZE]),
            current_frame_start: 0,
            len: 0,
        }
    }
}

impl<'stack> VirtualStack<'stack> {
    pub fn var(&self, rel_idx: u16) -> VirtualStackResult<&VirtualStackValue<'stack>> {
        if self.current_frame_start + rel_idx >= self.len {
            todo!()
        }
        Ok(&self.items[(self.current_frame_start + rel_idx) as usize])
    }

    pub fn var_mut(&mut self, rel_idx: u16) -> VirtualStackResult<&mut VirtualStackValue<'stack>> {
        if self.current_frame_start + rel_idx >= self.len {
            todo!()
        }
        Ok(&mut self.items[(self.current_frame_start + rel_idx) as usize])
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}
