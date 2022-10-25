use crate::*;

pub struct BorrowInstruction {
    variant: BorrowInstructionVariant,
}

impl BorrowInstruction {
    pub fn init<Idx>(idx: Idx) -> Self
    where
        Idx: Into<BorrowIdx>,
    {
        Self {
            variant: BorrowInstructionVariant::Init(idx.into()),
        }
    }
}

pub enum BorrowInstructionVariant {
    Init(BorrowIdx),
    Block(BorrowBlock),
    Loop(BorrowBlock),
}

pub struct BorrowBlock(Vec<BorrowInstruction>);

#[derive(Debug, Clone, Copy)]
pub enum BorrowIdx {
    Variable(VariableIdx),
    Lifetime(LifetimeIdx),
}

impl From<VariableIdx> for BorrowIdx {
    fn from(idx: VariableIdx) -> Self {
        BorrowIdx::Variable(idx)
    }
}

impl From<LifetimeIdx> for BorrowIdx {
    fn from(idx: LifetimeIdx) -> Self {
        BorrowIdx::Lifetime(idx)
    }
}

impl<'a> BorrowChecker<'a> {
    pub fn exec_all(&mut self, instrns: &[BorrowInstruction]) -> BorrowResult<()> {
        for instrn in instrns {
            self.exec(instrn)?
        }
        Ok(())
    }

    pub fn exec(&mut self, instrn: &BorrowInstruction) -> BorrowResult<()> {
        match instrn.variant {
            BorrowInstructionVariant::Init(idx) => todo!(),
            BorrowInstructionVariant::Block(_) => todo!(),
            BorrowInstructionVariant::Loop(_) => todo!(),
        }
    }
}
