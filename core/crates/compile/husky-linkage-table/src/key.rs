use crate::*;
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageKey {
    VecConstructor {
        element_ty_uid: EntityUid,
    },
    TypeCall {
        ty_uid: EntityUid,
    },
    Routine {
        routine_uid: EntityUid,
    },
    ElementAccess {
        opd_uids: SmallVec<[EntityUid; 2]>,
    },
    StructFieldAccess {
        this_ty_uid: EntityUid,
        field_ident: CustomIdentifier,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum StaticLinkageKey {
    VecConstructor {
        element_ty: &'static str,
    },
    TypeCall {
        ty: &'static str,
    },
    Routine {
        routine: &'static str,
    },
    ElementAccess {
        opd_uids: &'static [&'static str],
    },
    StructFieldAccess {
        this_ty: &'static str,
        field_ident: &'static str,
    },
}

impl LinkageKey {
    pub fn from_static(db: &dyn ResolveLinkage, static_key: StaticLinkageKey) -> Self {
        match static_key {
            StaticLinkageKey::VecConstructor { element_ty } => LinkageKey::VecConstructor {
                element_ty_uid: entity_uid(db, element_ty),
            },
            StaticLinkageKey::TypeCall { ty } => LinkageKey::TypeCall {
                ty_uid: entity_uid(db, ty),
            },
            StaticLinkageKey::Routine { routine } => LinkageKey::Routine {
                routine_uid: entity_uid(db, routine),
            },
            StaticLinkageKey::ElementAccess { opd_uids } => LinkageKey::ElementAccess {
                opd_uids: opd_uids
                    .iter()
                    .map(|opd_uid| entity_uid(db, opd_uid))
                    .collect(),
            },
            StaticLinkageKey::StructFieldAccess {
                this_ty,
                field_ident,
            } => LinkageKey::StructFieldAccess {
                this_ty_uid: entity_uid(db, this_ty),
                field_ident: db.custom_ident(field_ident),
            },
        }
    }
}
fn entity_uid(db: &dyn ResolveLinkage, text: &str) -> EntityUid {
    db.entity_uid(db.parse_entity_route(None, text))
}
