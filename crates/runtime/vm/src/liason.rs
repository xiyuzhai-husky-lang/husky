use crate::*;
use check_utils::should_eq;
use print_utils::p;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputLiason {
    Pure,
    GlobalRef,
    Move,
    BorrowMut,
    MoveMut,
    Exec,
    MemberAccess,
}

impl InputLiason {
    pub fn eager(
        self,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
        is_this: bool,
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
                    InputLiason::Pure => EagerContract::Pure,
                    InputLiason::GlobalRef => EagerContract::GlobalRef,
                    InputLiason::Move => EagerContract::Move,
                    InputLiason::BorrowMut => EagerContract::RefMut,
                    InputLiason::MoveMut => todo!(),
                    InputLiason::Exec => todo!(),
                    InputLiason::MemberAccess => panic!(),
                })
            }
            OutputLiason::MemberAccess => {
                if !is_this {
                    Ok(EagerContract::Pure)
                } else {
                    // this
                    Ok(match output_contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::GlobalRef => todo!(),
                        EagerContract::Move => todo!(),
                        EagerContract::LetInit => {
                            if is_output_ty_copyable {
                                EagerContract::Pure
                            } else {
                                EagerContract::UseMemberForLetInit
                            }
                        }
                        EagerContract::VarInit => todo!(),
                        EagerContract::UseMemberForLetInit => EagerContract::UseMemberForLetInit,
                        EagerContract::UseMemberForVarInit => todo!(),
                        EagerContract::Return => todo!(),
                        EagerContract::RefMut => output_contract,
                        EagerContract::MoveMut => todo!(),
                        EagerContract::Exec => todo!(),
                    })
                }
            }
        }
    }

    pub fn lazy(self, output: OutputLiason) -> VMCompileResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match self {
                InputLiason::Pure => LazyContract::Pure,
                InputLiason::GlobalRef => todo!(),
                InputLiason::Move => todo!(),
                InputLiason::BorrowMut => todo!(),
                InputLiason::MoveMut => todo!(),
                InputLiason::Exec => todo!(),
                InputLiason::MemberAccess => todo!(),
            }),
            OutputLiason::MemberAccess => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputLiason {
    Transfer,
    MemberAccess,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FieldLiason {
    Own,
    GlobalRef,
    LazyOwn,
}

impl FieldLiason {
    pub fn mutable(&self) -> bool {
        match self {
            FieldLiason::Own => true,
            FieldLiason::GlobalRef | FieldLiason::LazyOwn => false,
        }
    }

    pub fn constructor_input_liason(&self, is_copyable: bool) -> InputLiason {
        match self {
            FieldLiason::Own => {
                if is_copyable {
                    InputLiason::Pure
                } else {
                    InputLiason::Move
                }
            }
            FieldLiason::GlobalRef => InputLiason::GlobalRef,
            FieldLiason::LazyOwn => panic!(),
        }
    }

    pub fn this(&self, input_liason: EagerContract) -> VMCompileResult<EagerContract> {
        match self {
            FieldLiason::Own => match input_liason {
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
            FieldLiason::GlobalRef => todo!(),
            FieldLiason::LazyOwn => todo!(),
        }
    }
}
