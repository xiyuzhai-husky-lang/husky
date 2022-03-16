use vm::{Contract, InitKind};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Qual {
    PureInput,
    ImmutableOwned,
    MutableOwned,
    FrameVar,
}

impl Qual {
    pub fn from_input(contract: Contract) -> Self {
        match contract {
            Contract::PureInput => Qual::PureInput,
            Contract::Share => todo!(),
            Contract::Take => todo!(),
            Contract::BorrowMut => todo!(),
            Contract::TakeMut => todo!(),
        }
    }

    pub fn frame_var() -> Self {
        Self::FrameVar
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Owner {
    PureExternal,
    OnStack(u8),
}

pub struct QualifiedType {
    qual: Qual,
    ty: ScopePtr,
}

impl Qual {
    pub(crate) fn from_init(init_kind: InitKind) -> Self {
        match init_kind {
            InitKind::Let => Self::ImmutableOwned,
            InitKind::Var => Self::MutableOwned,
            InitKind::Decl => Self::ImmutableOwned,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct QualTable {
    next_owner_id_raw: u8,
}

impl QualTable {
    fn new_owner_on_stack(&mut self) -> Owner {
        let id = Owner::OnStack(self.next_owner_id_raw);
        self.next_owner_id_raw += 1;
        id
    }
}
