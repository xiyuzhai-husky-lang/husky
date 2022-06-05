use crate::*;
use entity_route::EntityRouteKind;
use infer_decl::DeclQueryGroup;
use infer_error::*;
use print_utils::msg_once;
use std::fmt::Write;
use test_utils::{TestDisplay, TestDisplayConfig};
use word::RootIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EagerQualifiedTy {
    pub qual: EagerQualifier,
    pub ty: EntityRoutePtr,
}

impl TestDisplay for EagerQualifiedTy {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}{: <12?}{} {}{:?}{}",
                print_utils::PINK,
                self.qual,
                print_utils::RESET,
                print_utils::GREEN,
                self.ty,
                print_utils::RESET,
            )
            .unwrap()
        } else {
            write!(result, "{: <12?} {:?}", self.qual, self.ty,).unwrap()
        }
    }
}

impl EagerQualifiedTy {
    pub(crate) fn ty_qualified_ty() -> Self {
        Self {
            qual: EagerQualifier::EvalRef,
            ty: EntityRoutePtr::Root(RootIdentifier::TypeType),
        }
    }
    pub(crate) fn from_parameter_use(
        db: &dyn InferQualifiedTyQueryGroup,
        parameter_liason: ParameterLiason,
        ty: EntityRoutePtr,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<Self> {
        Ok(EagerQualifiedTy {
            qual: EagerQualifier::parameter_use_eager_qualifier(
                db.upcast(),
                ty,
                parameter_liason,
                contract,
                range,
            )?,
            ty: ty.deref_route(),
        })
    }
    pub(crate) fn from_parameter(
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
    ) -> InferResult<Self> {
        Ok(EagerQualifiedTy {
            qual: EagerQualifier::parameter_eager_qualifier(db.upcast(), ty, parameter_liason)?,
            ty: ty.deref_route(),
        })
    }

    pub(crate) fn new(qual: EagerQualifier, ty: EntityRoutePtr) -> Self {
        match ty.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => todo!(),
            _ => Self { qual, ty },
        }
    }

    pub(crate) fn from_member(
        db: &dyn InferQualifiedTyQueryGroup,
        this_qual: EagerQualifier,
        field_ty: EntityRoutePtr,
        field_liason: MemberLiason,
        is_field_copyable: bool,
    ) -> InferResult<Self> {
        msg_once!("ad hoc; consider ref");
        Ok(Self::new(
            EagerQualifier::member(this_qual, field_liason, is_field_copyable),
            field_ty,
        ))
    }

    pub(crate) fn init_variable_qualified_ty(self, init_kind: InitKind) -> InferResult<Self> {
        let qual = match init_kind {
            InitKind::Let => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => EagerQualifier::Copyable,
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::TempRef => EagerQualifier::TempRef,
                EagerQualifier::Transient | EagerQualifier::OwnedMut => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::EvalRef => todo!(),
                EagerQualifier::TempRefMut => todo!(),
            },
            InitKind::Var => match self.qual {
                EagerQualifier::Copyable | EagerQualifier::CopyableMut => {
                    EagerQualifier::CopyableMut
                }
                EagerQualifier::PureRef => todo!(),
                EagerQualifier::TempRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::OwnedMut,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::EvalRef => EagerQualifier::CopyableMut,
                EagerQualifier::TempRefMut => todo!(),
            },
            InitKind::Decl => match self.qual {
                EagerQualifier::Copyable => EagerQualifier::Copyable,
                EagerQualifier::CopyableMut => panic!(),
                EagerQualifier::PureRef => EagerQualifier::PureRef,
                EagerQualifier::EvalRef => EagerQualifier::EvalRef,
                EagerQualifier::TempRef => todo!(),
                EagerQualifier::Transient => EagerQualifier::Owned,
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => panic!(),
                EagerQualifier::TempRefMut => todo!(),
            },
        };
        Ok(Self { qual, ty: self.ty })
    }

    pub fn is_implicitly_castable_to_output(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        output_liason: OutputLiason,
        output_ty: EntityRoutePtr,
    ) -> bool {
        if !db.is_implicitly_castable(self.ty, output_ty) {
            return false;
        }
        match output_liason {
            OutputLiason::Transfer => match self.qual {
                EagerQualifier::PureRef | EagerQualifier::TempRef => false,
                EagerQualifier::Transient
                | EagerQualifier::Copyable
                | EagerQualifier::CopyableMut
                | EagerQualifier::Owned
                | EagerQualifier::OwnedMut => true,
                EagerQualifier::EvalRef => todo!(),
                EagerQualifier::TempRefMut => todo!(),
            },
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn as_ty(
        self,
        db: &dyn InferQualifiedTyQueryGroup,
        ty: EntityRoutePtr,
    ) -> InferResult<Self> {
        if !db.is_explicitly_castable(self.ty, ty)? {
            todo!()
        }
        Ok(Self {
            qual: self.qual,
            ty,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EagerQualifier {
    Copyable,
    CopyableMut,
    Owned,
    OwnedMut,
    PureRef,
    EvalRef,
    TempRef,
    TempRefMut,
    Transient,
}

impl std::fmt::Debug for EagerQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(match self {
            EagerQualifier::Copyable => "Copyable",
            EagerQualifier::CopyableMut => "CopyableMut",
            EagerQualifier::Owned => "Owned",
            EagerQualifier::OwnedMut => "OwnedMut",
            EagerQualifier::PureRef => "PureRef",
            EagerQualifier::EvalRef => "EvalRef",
            EagerQualifier::TempRef => "TempRef",
            EagerQualifier::TempRefMut => "RefMut",
            EagerQualifier::Transient => "Transient",
        })
    }
}

impl EagerQualifier {
    pub fn mutable(&self) -> bool {
        match self {
            EagerQualifier::Copyable
            | EagerQualifier::PureRef
            | EagerQualifier::EvalRef
            | EagerQualifier::TempRef
            | EagerQualifier::Owned
            | EagerQualifier::Transient => false,
            EagerQualifier::CopyableMut | EagerQualifier::OwnedMut | EagerQualifier::TempRefMut => {
                true
            }
        }
    }

    pub fn binding(self, contract: EagerContract) -> Binding {
        match self {
            EagerQualifier::PureRef | EagerQualifier::TempRef => match contract {
                EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                EagerContract::Move => panic!(),
                EagerContract::TempMutRef => todo!(),
                EagerContract::EvalRef => panic!(),
                EagerContract::TempRef => todo!(),
            },
            EagerQualifier::Transient => todo!(),
            EagerQualifier::Copyable => Binding::Copy,
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => Binding::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::Move => Binding::Move,
                EagerContract::TempMutRef => Binding::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::Move => Binding::Move,
                EagerContract::TempMutRef => Binding::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => Binding::TempRef,
            },
            EagerQualifier::EvalRef => match contract {
                EagerContract::Pure => Binding::TempRef,
                EagerContract::EvalRef => Binding::EvalRef,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::TempRefMut => todo!(),
        }
    }

    pub fn method_opt_output_binding(
        self,
        output_liason: OutputLiason,
        output_contract: EagerContract,
        is_output_ty_copyable: bool,
    ) -> Option<Binding> {
        match output_liason {
            OutputLiason::Transfer => None,
            OutputLiason::MemberAccess { member_liason } => {
                Some(self.member_binding(member_liason, output_contract, is_output_ty_copyable))
            }
        }
    }

    pub fn member_binding(
        self,
        member_liason: MemberLiason,
        member_contract: EagerContract,
        is_member_ty_copyable: bool,
    ) -> Binding {
        if is_member_ty_copyable {
            match member_contract {
                EagerContract::Pure => Binding::Copy,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => match member_liason {
                    MemberLiason::Immutable => todo!(),
                    MemberLiason::Mutable => Binding::TempRefMut,
                    MemberLiason::Derived => todo!(),
                },
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            }
        } else {
            // non-copyable
            match self {
                EagerQualifier::Copyable => todo!(),
                EagerQualifier::CopyableMut => todo!(),
                EagerQualifier::Owned => todo!(),
                EagerQualifier::OwnedMut => todo!(),
                EagerQualifier::PureRef => match member_contract {
                    EagerContract::Pure | EagerContract::Pass => Binding::TempRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempMutRef => todo!(),
                    EagerContract::EvalRef => todo!(),
                },
                EagerQualifier::EvalRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::EvalRef => Binding::EvalRef,
                    EagerContract::Move => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::TempMutRef => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerQualifier::TempRef => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempMutRef => todo!(),
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => Binding::TempRef,
                },
                EagerQualifier::TempRefMut => match member_contract {
                    EagerContract::Pure => todo!(),
                    EagerContract::Move => todo!(),
                    EagerContract::TempMutRef => Binding::TempRefMut,
                    EagerContract::EvalRef => todo!(),
                    EagerContract::TempRef => todo!(),
                    EagerContract::Pass => todo!(),
                },
                EagerQualifier::Transient => todo!(),
            }
        }
    }

    pub fn parameter_use_eager_qualifier(
        db: &dyn DeclQueryGroup,
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
        contract: EagerContract,
        range: TextRange,
    ) -> InferResult<Self> {
        Self::parameter_eager_qualifier(db, parameter_ty, parameter_liason)?
            .variable_use(contract, range)
    }

    pub fn parameter_eager_qualifier(
        db: &dyn DeclQueryGroup,
        parameter_ty: EntityRoutePtr,
        parameter_liason: ParameterLiason,
    ) -> InferResult<Self> {
        Ok(match parameter_ty.kind {
            EntityRouteKind::Root {
                ident: RootIdentifier::Ref,
            } => {
                if parameter_ty.temporal_arguments.len() == 0 {
                    EagerQualifier::EvalRef
                } else {
                    todo!()
                }
            }
            _ => {
                let is_copyable = db.is_copyable(parameter_ty)?;
                match parameter_liason {
                    ParameterLiason::Pure => {
                        if is_copyable {
                            EagerQualifier::Copyable
                        } else {
                            EagerQualifier::PureRef
                        }
                    }
                    ParameterLiason::EvalRef => EagerQualifier::EvalRef,
                    ParameterLiason::Move => EagerQualifier::Owned,
                    ParameterLiason::TempRefMut => todo!(),
                    ParameterLiason::MoveMut => todo!(),
                    ParameterLiason::MemberAccess => todo!(),
                    ParameterLiason::TempRef => todo!(),
                }
            }
        })
    }

    // pub fn field(
    //     this_qual: EagerQualifier,
    //     field_liason: MemberLiason,
    //     is_field_copyable: bool,
    // ) -> InferResult<Self> {
    //     Ok(if is_field_copyable {
    //         if this_qual.mutable() && field_liason.mutable() {
    //             EagerQualifier::CopyableMut
    //         } else {
    //             EagerQualifier::Copyable
    //         }
    //     } else {
    //         // non-copyable
    //         match this_qual {
    //             EagerQualifier::Copyable | EagerQualifier::CopyableMut => panic!(),
    //             EagerQualifier::PureRef => EagerQualifier::PureRef,
    //             EagerQualifier::EvalRef => EagerQualifier::EvalRef,
    //             EagerQualifier::TempRef => EagerQualifier::TempRef,
    //             EagerQualifier::Transient => match field_liason {
    //                 MemberLiason::Immutable => todo!(),
    //                 MemberLiason::Mutable => todo!(),
    //                 MemberLiason::Derived => todo!(),
    //             },
    //             EagerQualifier::Owned | EagerQualifier::OwnedMut => panic!(),
    //             // match field_liason {
    //             //     FieldLiason::Mutable => EagerQualifier::TempRefMut,
    //             //     FieldLiason::Immutable => EagerQualifier::TempRef,
    //             //     FieldLiason::Derived => todo!(),
    //             // },
    //             EagerQualifier::TempRefMut => match field_liason {
    //                 MemberLiason::Mutable => EagerQualifier::TempRefMut,
    //                 MemberLiason::Immutable => panic!("shouldn't be here"),
    //                 MemberLiason::Derived => todo!(),
    //             },
    //         }
    //     })
    // }

    pub fn from_output(output_liason: OutputLiason, is_copyable: bool) -> Self {
        match output_liason {
            OutputLiason::Transfer => Self::transitive(is_copyable),
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    pub fn variable_use(self, contract: EagerContract, range: TextRange) -> InferResult<Self> {
        Ok(match self {
            EagerQualifier::Copyable => match contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => EagerQualifier::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::CopyableMut => match contract {
                EagerContract::Pure => EagerQualifier::Copyable,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => EagerQualifier::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::Owned => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => EagerQualifier::Transient,
                EagerContract::TempMutRef => todo!(),
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::OwnedMut => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => EagerQualifier::Transient,
                EagerContract::TempMutRef => EagerQualifier::TempRefMut,
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => EagerQualifier::TempRef,
            },
            EagerQualifier::PureRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => {
                    return throw!(format!("can't move from a pure ref",), range)
                }
                EagerContract::TempMutRef => todo!(),
                EagerContract::EvalRef => {
                    throw!(format!("can't turn a pure ref to a eval ref",), range)
                }
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => EagerQualifier::PureRef,
            },
            EagerQualifier::EvalRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => {
                    throw!(format!("can't move from an eval ref",), range)
                }
                EagerContract::TempMutRef => {
                    throw!(format!("can't turn eval ref into a temp mut ref",), range)
                }
                EagerContract::EvalRef => EagerQualifier::EvalRef,
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => EagerQualifier::EvalRef,
            },
            EagerQualifier::TempRef => match contract {
                EagerContract::Pure => EagerQualifier::PureRef,
                EagerContract::Move => todo!(),
                EagerContract::TempMutRef => todo!(),
                EagerContract::Pure => todo!(),
                EagerContract::EvalRef => todo!(),
                EagerContract::TempRef => todo!(),
                EagerContract::Pass => todo!(),
            },
            EagerQualifier::TempRefMut => todo!(),
            EagerQualifier::Transient => todo!(),
        })
    }

    pub fn transitive(is_copyable: bool) -> Self {
        if is_copyable {
            EagerQualifier::Copyable
        } else {
            EagerQualifier::Transient
        }
    }

    pub fn member(this_qual: Self, field_liason: MemberLiason, is_member_copyable: bool) -> Self {
        if is_member_copyable {
            match this_qual {
                EagerQualifier::Copyable
                | EagerQualifier::PureRef
                | EagerQualifier::EvalRef
                | EagerQualifier::TempRef
                | EagerQualifier::Transient => EagerQualifier::Copyable,
                EagerQualifier::CopyableMut | EagerQualifier::Owned | EagerQualifier::OwnedMut => {
                    panic!()
                }
                EagerQualifier::TempRefMut => EagerQualifier::TempRefMut,
            }
        } else {
            match this_qual {
                EagerQualifier::Copyable
                | EagerQualifier::CopyableMut
                | EagerQualifier::Owned
                | EagerQualifier::OwnedMut => panic!(),
                EagerQualifier::PureRef
                | EagerQualifier::EvalRef
                | EagerQualifier::TempRef
                | EagerQualifier::Transient
                | EagerQualifier::TempRefMut => this_qual,
            }
        }
    }
}
