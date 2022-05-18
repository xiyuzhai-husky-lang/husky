use check_utils::should_eq;

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
pub enum OutputLiason {
    Transfer,
    MemberAccess,
}

impl InputContract {
    pub fn eager(
        self,
        output_liason: OutputLiason,
        output_contract: EagerContract,
    ) -> VMCompileResult<EagerContract> {
        match output_liason {
            OutputLiason::Transfer => {
                match output_contract {
                    EagerContract::Pure
                    | EagerContract::Move
                    | EagerContract::Return
                    | EagerContract::LetInit
                    | EagerContract::VarInit
                    | EagerContract::Exec => (),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::RefMut => match output_liason {
                        OutputLiason::Transfer => {
                            return Err(vm_compile_error!(format!(
                                "can't mutate transferred output"
                            )))
                        }
                        OutputLiason::MemberAccess => todo!(),
                    },
                    EagerContract::MoveMut => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                }
                Ok(match self {
                    InputContract::Pure => EagerContract::Pure,
                    InputContract::GlobalRef => EagerContract::GlobalRef,
                    InputContract::Move => EagerContract::Move,
                    InputContract::BorrowMut => EagerContract::RefMut,
                    InputContract::MoveMut => todo!(),
                    InputContract::Exec => todo!(),
                    InputContract::MemberAccess => panic!(),
                })
            }
            OutputLiason::MemberAccess => {
                should_eq!(self, InputContract::MemberAccess);
                match output_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::GlobalRef => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::LetInit => todo!(),
                    EagerContract::VarInit => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::RefMut => Ok(output_contract),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                }
            }
        }
    }

    pub fn lazy(self, output: OutputLiason) -> VMCompileResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match self {
                InputContract::Pure => LazyContract::Pure,
                InputContract::GlobalRef => todo!(),
                InputContract::Move => todo!(),
                InputContract::BorrowMut => todo!(),
                InputContract::MoveMut => todo!(),
                InputContract::Exec => todo!(),
                InputContract::MemberAccess => todo!(),
            }),
            OutputLiason::MemberAccess => todo!(),
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
    Init,
    Return,
    UseMemberForInit,
    UseMemberForReturn,
    GlobalRef,
    Pure,
    Move,
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

    pub fn this(&self, input_contract: EagerContract) -> VMCompileResult<EagerContract> {
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
