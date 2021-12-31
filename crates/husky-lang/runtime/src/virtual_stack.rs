mod impl_exec;
mod item;
#[cfg(test)]
mod tests;

use hir::*;

use crate::*;
use item::VirtualStackItem;

pub struct VirtualStack {
    items: Box<[VirtualStackItem; VIRTUAL_STACK_SIZE]>,
    current_frame_start: u16,
    len: u16,
}

impl std::fmt::Debug for VirtualStack {
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

impl VirtualStack {
    pub fn new() -> Self {
        const DEFAULT_VIRTUAL_STACK_ITEM: VirtualStackItem = VirtualStackItem::Undefined;
        VirtualStack {
            items: Box::new([DEFAULT_VIRTUAL_STACK_ITEM; VIRTUAL_STACK_SIZE]),
            current_frame_start: 0,
            len: 0,
        }
    }
}

impl VirtualStack {
    pub fn var(&self, rel_idx: u16) -> RuntimeResult<&VirtualStackItem> {
        if self.current_frame_start + rel_idx >= self.len {
            todo!()
        }
        Ok(&self.items[(self.current_frame_start + rel_idx) as usize])
    }

    pub fn var_mut(&mut self, rel_idx: u16) -> RuntimeResult<&mut VirtualStackItem> {
        if self.current_frame_start + rel_idx >= self.len {
            todo!()
        }
        Ok(&mut self.items[(self.current_frame_start + rel_idx) as usize])
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}
