use crate::*;
use check_utils::should_eq;
use print_utils::p;
use word::LiasonKeyword;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParameterLiason {
    Pure,
    Move,
    TempRefMut,
    MoveMut,
    MemberAccess,
    EvalRef,
}

impl ParameterLiason {
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
                    EagerContract::TempRefMut => match output_liason {
                        OutputLiason::Transfer => {
                            return Err(vm_compile_error!(format!(
                                "can't mutate transferred output"
                            )))
                        }
                        OutputLiason::MemberAccess { .. } => todo!(),
                    },
                    EagerContract::MoveMut => todo!(),
                    EagerContract::UseMemberForLetInit => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::EvalRef => todo!(),
                }
                Ok(match self {
                    ParameterLiason::Pure => EagerContract::Pure,
                    ParameterLiason::Move | ParameterLiason::MoveMut => EagerContract::Move,
                    ParameterLiason::TempRefMut => EagerContract::TempRefMut,
                    ParameterLiason::MemberAccess => panic!(),
                    ParameterLiason::EvalRef => EagerContract::EvalRef,
                })
            }
            OutputLiason::MemberAccess { .. } => {
                if !is_this {
                    Ok(EagerContract::Pure)
                } else {
                    // this
                    Ok(match output_contract {
                        EagerContract::Pure => EagerContract::Pure,
                        EagerContract::Move => EagerContract::Move,
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
                        EagerContract::TempRefMut => output_contract,
                        EagerContract::MoveMut => todo!(),
                        EagerContract::Exec => todo!(),
                        EagerContract::UseForAssignRvalue => todo!(),
                        EagerContract::EvalRef => todo!(),
                    })
                }
            }
        }
    }

    pub fn lazy(self, output: OutputLiason) -> VMCompileResult<LazyContract> {
        match output {
            OutputLiason::Transfer => Ok(match self {
                ParameterLiason::Pure => LazyContract::Pure,
                ParameterLiason::Move => todo!(),
                ParameterLiason::TempRefMut => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => todo!(),
            }),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OutputLiason {
    Transfer,
    MemberAccess { member_liason: MemberLiason },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MemberLiason {
    Immutable,
    Mutable,
    Derived,
}

impl MemberLiason {
    pub fn from_opt_keyword(opt_keyword: Option<LiasonKeyword>) -> MemberLiason {
        match opt_keyword {
            Some(liason_keyword) => match liason_keyword {
                LiasonKeyword::Mut => MemberLiason::Mutable,
            },
            None => MemberLiason::Immutable,
        }
    }

    pub fn mutable(self) -> bool {
        match self {
            MemberLiason::Immutable | MemberLiason::Derived => false,
            MemberLiason::Mutable => true,
        }
    }

    pub fn constructor_input_liason(self, is_copyable: bool) -> ParameterLiason {
        match self {
            MemberLiason::Immutable => {
                if is_copyable {
                    ParameterLiason::Pure
                } else {
                    ParameterLiason::Move
                }
            }
            MemberLiason::Mutable => {
                if is_copyable {
                    ParameterLiason::Pure
                } else {
                    ParameterLiason::MoveMut
                }
            }
            MemberLiason::Derived => panic!(),
        }
    }

    pub fn this_eager_contract(
        self,
        member_contract: EagerContract,
        is_member_copyable: bool,
    ) -> VMCompileResult<EagerContract> {
        if is_member_copyable {
            Ok(match member_contract {
                EagerContract::Pure => EagerContract::Pure,
                EagerContract::Move => todo!(),
                EagerContract::UseForLetInit => EagerContract::Pure,
                EagerContract::UseForVarInit => EagerContract::Pure,
                EagerContract::UseForAssignRvalue => todo!(),
                EagerContract::UseMemberForLetInit => todo!(),
                EagerContract::UseMemberForVarInit => todo!(),
                EagerContract::Return => EagerContract::Pure,
                EagerContract::TempRefMut => match self {
                    MemberLiason::Immutable => {
                        return Err(vm_compile_error!(format!(
                            "can't turn an immutable member into ref mut"
                        )))
                    }
                    MemberLiason::Mutable => EagerContract::TempRefMut,
                    MemberLiason::Derived => todo!(),
                },
                EagerContract::MoveMut => todo!(),
                EagerContract::Exec => todo!(),
                EagerContract::EvalRef => todo!(),
            })
        } else {
            match self {
                MemberLiason::Immutable => match member_contract {
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
                    EagerContract::TempRefMut => Err(vm_compile_error!(format!(
                        "can't bind mutable reference to an immutable field"
                    ))),
                    EagerContract::UseForAssignRvalue => Err(vm_compile_error!(format!(
                        "can't assign to an immutable field"
                    ))),
                    EagerContract::EvalRef => Ok(EagerContract::EvalRef),
                },
                MemberLiason::Mutable => match member_contract {
                    EagerContract::Pure => Ok(EagerContract::Pure),
                    EagerContract::Move => Ok(EagerContract::Move),
                    EagerContract::TempRefMut => Ok(EagerContract::TempRefMut),
                    EagerContract::MoveMut => todo!(),
                    EagerContract::Exec => todo!(),
                    EagerContract::UseForLetInit | EagerContract::UseMemberForLetInit => {
                        Ok(EagerContract::UseMemberForLetInit)
                    }
                    EagerContract::UseForVarInit => todo!(),
                    EagerContract::Return => todo!(),
                    EagerContract::UseMemberForVarInit => todo!(),
                    EagerContract::UseForAssignRvalue => todo!(),
                    EagerContract::EvalRef => todo!(),
                },
                MemberLiason::Derived => panic!(),
            }
        }
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
        //     FieldLiason::EvalRef => todo!(),
        //     FieldLiason::LazyOwn => todo!(),
        // };
    }

    pub fn this_lazy_contract(
        self,
        field_contract: LazyContract,
        is_member_copyable: bool,
    ) -> VMCompileResult<LazyContract> {
        Ok(if is_member_copyable {
            match field_contract {
                LazyContract::Init => todo!(),
                LazyContract::Return => todo!(),
                LazyContract::UseMemberForInit => todo!(),
                LazyContract::UseMemberForReturn => todo!(),
                LazyContract::EvalRef => todo!(),
                LazyContract::Pure => LazyContract::Pure,
                LazyContract::Move => todo!(),
            }
        } else {
            match self {
                MemberLiason::Immutable | MemberLiason::Mutable => match field_contract {
                    LazyContract::Move => LazyContract::Move,
                    LazyContract::Pure => LazyContract::Pure,
                    LazyContract::EvalRef => todo!(),
                    LazyContract::Init => todo!(),
                    LazyContract::Return => todo!(),
                    LazyContract::UseMemberForInit => todo!(),
                    LazyContract::UseMemberForReturn => todo!(),
                },
                MemberLiason::Derived => todo!(),
            }
        })
    }
}
