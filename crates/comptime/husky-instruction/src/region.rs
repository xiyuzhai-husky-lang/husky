use crate::{*};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionRegion {
    Block(Instructions),
}
