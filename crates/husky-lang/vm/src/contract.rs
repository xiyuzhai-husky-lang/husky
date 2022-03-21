use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    Ref,
    Take,
    BorrowMut,
    TakeMut,
    Exec,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyContract {
    Take,
    Ref,
    Pure,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MembVarContract {
    Own,
    Ref,
}

impl MembVarContract {
    pub fn constructor_input(&self) -> EagerContract {
        match self {
            MembVarContract::Own => EagerContract::Take,
            MembVarContract::Ref => EagerContract::Ref,
        }
    }

    pub fn this(&self, input_contract: EagerContract) -> VMResult<EagerContract> {
        match self {
            MembVarContract::Own => match input_contract {
                EagerContract::Pure => todo!(),
                EagerContract::Ref => todo!(),
                EagerContract::Take => Ok(EagerContract::Take),
                EagerContract::BorrowMut => Ok(EagerContract::BorrowMut),
                EagerContract::TakeMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            MembVarContract::Ref => todo!(),
        }
    }
}
