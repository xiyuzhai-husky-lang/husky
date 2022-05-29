use crate::*;
use check_utils::should_eq;
use print_utils::p;
use word::LiasonKeyword;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputLiason {
    Pure,
    Move,
    BorrowMut,
    MoveMut,
    MemberAccess,
    GlobalRef,
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
                    | EagerContract::UseForLetInit
                    | EagerContract::UseForVarInit
                    | EagerContract::Exec
                    | EagerContract::UseForAssignRvalue => (),
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
                    InputLiason::Move => EagerContract::Move,
                    InputLiason::BorrowMut => EagerContract::RefMut,
                    InputLiason::MoveMut => todo!(),
                    InputLiason::MemberAccess => panic!(),
                    InputLiason::GlobalRef => todo!(),
                })
            }
            OutputLiason::MemberAccess => {
                if !is_this {
                    Ok(EagerContract::Pure)
                } else {
                    // this
                    Ok(match output_contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::Move => todo!(),
                        EagerContract::UseForLetInit => {
                            if is_output_ty_copyable {
                                EagerContract::Pure
                            } else {
                                EagerContract::UseMemberForLetInit
                            }
                        }
                        EagerContract::UseForVarInit => todo!(),
                        EagerContract::UseMemberForLetInit => EagerContract::UseMemberForLetInit,
                        EagerContract::UseMemberForVarInit => todo!(),
                        EagerContract::Return => todo!(),
                        EagerContract::RefMut => output_contract,
                        EagerContract::MoveMut => todo!(),
                        EagerContract::Exec => todo!(),
                        EagerContract::UseForAssignRvalue => todo!(),
                    })
                }
            }
        }
    }

    pub fn lazy(self, output: OutputLiason) -> VMCompileResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match self {
                InputLiason::Pure => LazyContract::Pure,
                InputLiason::Move => todo!(),
                InputLiason::BorrowMut => todo!(),
                InputLiason::MoveMut => todo!(),
                InputLiason::MemberAccess => todo!(),
                InputLiason::GlobalRef => todo!(),
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
    Immutable,
    Mutable,
    Derived,
}

impl FieldLiason {
    pub fn from_opt_keyword(opt_keyword: Option<LiasonKeyword>) -> FieldLiason {
        match opt_keyword {
            Some(liason_keyword) => match liason_keyword {
                LiasonKeyword::Mut => FieldLiason::Mutable,
            },
            None => FieldLiason::Immutable,
        }
    }

    pub fn mutable(self) -> bool {
        match self {
            FieldLiason::Immutable | FieldLiason::Derived => false,
            FieldLiason::Mutable => true,
        }
    }

    pub fn constructor_input_liason(&self, is_copyable: bool) -> InputLiason {
        match self {
            FieldLiason::Immutable => {
                if is_copyable {
                    InputLiason::Pure
                } else {
                    InputLiason::Move
                }
            }
            FieldLiason::Mutable => {
                if is_copyable {
                    InputLiason::Pure
                } else {
                    InputLiason::MoveMut
                }
            }
            FieldLiason::Derived => panic!(),
        }
    }

    pub fn this_eager_contract(
        self,
        field_contract: EagerContract,
    ) -> VMCompileResult<EagerContract> {
        //  match field_decl.liason {
        //     FieldLiason::Own => match contract {
        //         EagerContract::Pure => EagerContract::Pure,
        //
        //         EagerContract::Move => EagerContract::Move,
        //         EagerContract::Return => {
        //             if self.db.is_copyable(field_decl.ty)? {
        //                 EagerContract::Pure
        //             } else {
        //                 todo!()
        //             }
        //         }
        //         EagerContract::RefMut => EagerContract::RefMut,
        //         EagerContract::MoveMut => todo!(),
        //         EagerContract::Exec => todo!(),
        //         EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
        //             EagerContract::UseMemberForLetInit
        //         }
        //         EagerContract::UseForVarInit => todo!(),
        //         EagerContract::UseMemberForVarInit => EagerContract::UseMemberForVarInit,
        //         EagerContract::UseForAssign => throw!(
        //             format!(
        //             "can't use noncopyable field for assignment without explicit moving"
        //         ),
        //             arena[raw_expr_idx].range
        //         ),
        //     },
        //     FieldLiason::GlobalRef => todo!(),
        //     FieldLiason::LazyOwn => todo!(),
        // };
        match self {
            FieldLiason::Immutable => match field_contract {
                EagerContract::Pure => Ok(EagerContract::Pure),

                EagerContract::Move | EagerContract::MoveMut => Ok(EagerContract::Move),
                EagerContract::Exec => todo!(),
                EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                    Ok(EagerContract::UseMemberForLetInit)
                }
                EagerContract::UseForVarInit | EagerContract::UseMemberForVarInit => {
                    Ok(EagerContract::UseMemberForVarInit)
                }
                EagerContract::Return => todo!(),
                EagerContract::RefMut => Err(vm_compile_error!(format!(
                    "can't bind mutable reference to an immutable field"
                ))),
                EagerContract::UseForAssignRvalue => Err(vm_compile_error!(format!(
                    "can't assign to an immutable field"
                ))),
            },
            FieldLiason::Mutable => match field_contract {
                EagerContract::Pure => todo!(),

                EagerContract::Move => Ok(EagerContract::Move),
                EagerContract::RefMut => Ok(EagerContract::RefMut),
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::UseForLetInit => todo!(),
                EagerContract::UseForVarInit => todo!(),
                EagerContract::Return => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::UseForAssignRvalue => todo!(),
            },
            FieldLiason::Derived => panic!(),
        }
    }

    pub fn this_lazy_contract(self, field_contract: LazyContract) -> VMCompileResult<LazyContract> {
        match self {
            FieldLiason::Immutable | FieldLiason::Mutable => match field_contract {
                LazyContract::Move => Ok(LazyContract::Move),
                LazyContract::Pure => Ok(LazyContract::Pure),
                LazyContract::GlobalRef => todo!(),
                LazyContract::Init => todo!(),
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
            },
            FieldLiason::Derived => panic!(),
        }
    }
}
