use crate::*;
use check_utils::should_eq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EagerContract {
    Pure,
    GlobalRef,
    Move,
    UseForLetInit,
    UseForVarInit,
    UseForAssignRvalue,
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
