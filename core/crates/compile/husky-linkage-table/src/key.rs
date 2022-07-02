use crate::*;
use smallvec::SmallVec;
use static_defn::__StaticLinkageKey;

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

impl LinkageKey {
    pub fn from_static(db: &dyn ResolveLinkage, static_key: __StaticLinkageKey) -> Self {
        match static_key {
            __StaticLinkageKey::VecConstructor { element_ty } => LinkageKey::VecConstructor {
                element_ty_uid: entity_uid(db, element_ty),
            },
            __StaticLinkageKey::TypeCall { ty } => LinkageKey::TypeCall {
                ty_uid: entity_uid(db, ty),
            },
            __StaticLinkageKey::Routine { routine } => LinkageKey::Routine {
                routine_uid: entity_uid(db, routine),
            },
            __StaticLinkageKey::ElementAccess { opd_uids } => LinkageKey::ElementAccess {
                opd_uids: opd_uids
                    .iter()
                    .map(|opd_uid| entity_uid(db, opd_uid))
                    .collect(),
            },
            __StaticLinkageKey::StructFieldAccess {
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
    db.entity_uid(db.parse_entity_route(db.opt_package_main(), text))
}
