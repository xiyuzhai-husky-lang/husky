use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputContract {
    Pure,
    GlobalRef,
    Move,
    BorrowMut,
    MoveMut,
    Exec,
    MemberAccess,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputContract {
    Transfer,
    MemberAccess,
}

impl InputContract {
    pub fn eager(&self, output: OutputContract) -> VMResult<EagerContract> {
        match output {
            OutputContract::Transfer => Ok(match self {
                InputContract::Pure => EagerContract::Pure,
                InputContract::GlobalRef => EagerContract::GlobalRef,
                InputContract::Move => EagerContract::Move,
                InputContract::BorrowMut => EagerContract::RefMut,
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
                InputContract::MemberAccess => panic!(),
            }),
            OutputContract::MemberAccess => todo!(),
        }
    }

    pub fn lazy(&self, output: OutputContract) -> VMResult<LazyContract> {
        match output {
            OutputContract::Transfer => Ok(match self {
                InputContract::Pure => LazyContract::Pure,
                InputContract::GlobalRef => todo!(),
                InputContract::Move => LazyContract::Move,
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
                InputContract::MemberAccess => todo!(),
            }),
            OutputContract::MemberAccess => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    GlobalRef,
    Move,
    LetInit,
    VarInit,
    UseMemberForLetInit,
    UseMemberForVarInit,
    Return,
    RefMut,
    MoveMut,
    Exec,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyContract {
    Move,
    GlobalRef,
    Pure,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FieldContract {
    Own,
    GlobalRef,
    LazyOwn,
}

impl FieldContract {
    pub fn mutable(&self) -> bool {
        match self {
            FieldContract::Own => true,
            FieldContract::GlobalRef | FieldContract::LazyOwn => false,
        }
    }

    pub fn constructor_input_contract(&self, is_copyable: bool) -> InputContract {
        match self {
            FieldContract::Own => {
                if is_copyable {
                    InputContract::Pure
                } else {
                    InputContract::Move
                }
            }
            FieldContract::GlobalRef => InputContract::GlobalRef,
            FieldContract::LazyOwn => panic!(),
        }
    }

    pub fn this(&self, input_contract: EagerContract) -> VMResult<EagerContract> {
        match self {
            FieldContract::Own => match input_contract {
                EagerContract::Pure => todo!(),
                EagerContract::GlobalRef => todo!(),
                EagerContract::Move => Ok(EagerContract::Move),
                EagerContract::RefMut => Ok(EagerContract::RefMut),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::LetInit => todo!(),
                EagerContract::VarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
            },
            FieldContract::GlobalRef => todo!(),
            FieldContract::LazyOwn => todo!(),
        }
    }
}
