use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Pure,
    GlobalRef,
    Take,
    BorrowMut,
    TakeMut,
    Exec,
}

impl InputContract {
    pub fn eager(&self) -> VMResult<EagerContract> {
        Ok(match self {
            InputContract::Pure => EagerContract::Pure,
            InputContract::GlobalRef => EagerContract::GlobalRef,
            InputContract::Take => EagerContract::Take,
            InputContract::BorrowMut => todo!(),
            InputContract::TakeMut => todo!(),
            InputContract::Exec => todo!(),
        })
    }

    pub fn lazy(&self) -> VMResult<LazyContract> {
        Ok(match self {
            InputContract::Pure => LazyContract::Pure,
            InputContract::GlobalRef => todo!(),
            InputContract::Take => todo!(),
            InputContract::BorrowMut => todo!(),
            InputContract::TakeMut => todo!(),
            InputContract::Exec => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    GlobalRef,
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
    pub fn constructor_input(&self) -> InputContract {
        match self {
            MembVarContract::Own => InputContract::Take,
            MembVarContract::Ref => InputContract::GlobalRef,
        }
    }

    pub fn this(&self, input_contract: EagerContract) -> VMResult<EagerContract> {
        match self {
            MembVarContract::Own => match input_contract {
                EagerContract::Pure => todo!(),
                EagerContract::GlobalRef => todo!(),
                EagerContract::Take => Ok(EagerContract::Take),
                EagerContract::BorrowMut => Ok(EagerContract::BorrowMut),
                EagerContract::TakeMut => todo!(),
                EagerContract::Exec => todo!(),
            },
            MembVarContract::Ref => todo!(),
        }
    }
}
