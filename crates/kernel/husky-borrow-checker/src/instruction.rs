use crate::*;

pub struct BorrowInstruction {
    variant: BorrowInstructionVariant,
}

impl BorrowInstruction {
    pub fn push<Idx>(idx: Idx) -> Self
    where
        Idx: Into<BorrowObject>,
    {
        Self {
            variant: BorrowInstructionVariant::Push(idx.into()),
        }
    }
}

pub enum BorrowInstructionVariant {
    Push(BorrowObject),
    Block(BorrowBlock),
    Loop(BorrowBlock),
}

pub struct BorrowBlock(Vec<BorrowInstruction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorrowObject {
    Variable(VariableIdx),
    Lifetime(LifetimeIdx),
}

impl From<VariableIdx> for BorrowObject {
    fn from(idx: VariableIdx) -> Self {
        BorrowObject::Variable(idx)
    }
}

impl From<LifetimeIdx> for BorrowObject {
    fn from(idx: LifetimeIdx) -> Self {
        BorrowObject::Lifetime(idx)
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
            BorrowInstructionVariant::Push(idx) => match idx {
                BorrowObject::Variable(idx) => Ok(self.init_variable(idx)),
                BorrowObject::Lifetime(idx) => Ok(self.push_lifetime(idx)),
            },
            BorrowInstructionVariant::Block(_) => todo!(),
            BorrowInstructionVariant::Loop(_) => todo!(),
        }
    }
}
