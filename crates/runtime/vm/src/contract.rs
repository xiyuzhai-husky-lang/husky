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
            InputContract::Take => EagerContract::Move,
            InputContract::BorrowMut => todo!(),
            InputContract::TakeMut => todo!(),
            InputContract::Exec => todo!(),
        })
    }

    pub fn lazy(&self) -> VMResult<LazyContract> {
        Ok(match self {
            InputContract::Pure => LazyContract::Pure,
            InputContract::GlobalRef => todo!(),
            InputContract::Take => LazyContract::Take,
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
    Move,
    LetInit,
    VarInit,
    Return,
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
pub enum MembAccessContract {
    Own,
    Ref,
    LazyOwn,
}

impl MembAccessContract {
    pub fn constructor_input(&self) -> InputContract {
        match self {
            MembAccessContract::Own => InputContract::Take,
            MembAccessContract::Ref => InputContract::GlobalRef,
            MembAccessContract::LazyOwn => panic!(),
        }
    }

    pub fn this(&self, input_contract: EagerContract) -> VMResult<EagerContract> {
        match self {
            MembAccessContract::Own => match input_contract {
                EagerContract::Pure => todo!(),
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => Ok(EagerContract::Move),
                EagerContract::BorrowMut => Ok(EagerContract::BorrowMut),
                EagerContract::TakeMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::Return => todo!(),
            },
            MembAccessContract::Ref => todo!(),
            MembAccessContract::LazyOwn => todo!(),
        }
    }
}
