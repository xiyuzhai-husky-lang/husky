use crate::*;
use check_utils::should_eq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    EvalRef,
    Move,
    UseForLetInit,
    UseForVarInit,
    UseForAssignRvalue,
    UseMemberForLetInit,
    UseMemberForVarInit,
    Return,
    TempRefMut,
    MoveMut,
    Exec,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LazyContract {
    Init,
    Return,
    UseMemberForInit,
    UseMemberForReturn,
    EvalRef,
    Pure,
    Move,
}
