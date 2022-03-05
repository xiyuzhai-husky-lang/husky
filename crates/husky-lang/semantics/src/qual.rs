use vm::InitKind;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qual {
    pub mutable: bool,
    pub owned: bool,
    pub owner_id: OwnerId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerId(u8);

pub struct QualifiedType {
    qual: Qual,
    ty: ScopePtr,
}

impl Qual {
    pub(crate) fn from_init(init_kind: InitKind, table: &mut QualTable) -> Self {
        match init_kind {
            InitKind::Let => Self {
                mutable: false,
                owned: true,
                owner_id: table.issue_owner_id(),
            },
            InitKind::Var => Self {
                mutable: true,
                owned: true,
                owner_id: table.issue_owner_id(),
            },
            InitKind::Decl => Self {
                mutable: false,
                owned: true,
                owner_id: table.issue_owner_id(),
            },
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct QualTable {
    next_owner_id_raw: u8,
}

impl QualTable {
    fn issue_owner_id(&mut self) -> OwnerId {
        let id = OwnerId(self.next_owner_id_raw);
        self.next_owner_id_raw += 1;
        id
    }
}
