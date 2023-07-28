use crate::*;

#[salsa::tracked(db = InstructionDb, jar = InstructionJar)]
pub struct Instructions {
    #[return_ref]
    data: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionBlockData {}
