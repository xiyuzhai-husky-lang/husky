use crate::*;

pub struct BorrowInstruction {
    variant: BorrowInstructionVariant,
}

impl BorrowInstruction {
    pub fn push<Idx>(idx: Idx) -> Self
    where
        Idx: Into<BorrowIdx>,
    {
        Self {
            variant: BorrowInstructionVariant::Push(idx.into()),
        }
    }
}

pub enum BorrowInstructionVariant {
    Push(BorrowIdx),
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
            BorrowInstructionVariant::Push(idx) => match idx {
                BorrowIdx::Variable(idx) => Ok(self.init_variable(idx)),
                BorrowIdx::Lifetime(idx) => Ok(self.push_lifetime(idx)),
            },
            BorrowInstructionVariant::Block(_) => todo!(),
            BorrowInstructionVariant::Loop(_) => todo!(),
        }
    }
}
