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
    FeatureEagerBlock {
        uid: EntityUid,
    },
    Routine {
        routine_uid: EntityUid,
    },
    Index {
        opd_uids: SmallVec<[EntityUid; 2]>,
    },
    StructField {
        this_ty_uid: EntityUid,
        field_ident: Ident,
    },
}

impl LinkageKey {
    // pub fn from_static(db: &dyn ResolveLinkage, static_key: __StaticLinkageKey) -> Self {
    //     match static_key {
    //         __StaticLinkageKey::VecConstructor { element_ty } => LinkageKey::VecConstructor {
    //             element_ty_uid: entity_uid(db, element_ty),
    //         },
    //         __StaticLinkageKey::TypeCall { ty } => LinkageKey::TypeCall {
    //             ty_uid: entity_uid(db, ty),
    //         },
    //         __StaticLinkageKey::Routine { route } => LinkageKey::Routine {
    //             routine_uid: entity_uid(db, route),
    //         },
    //         __StaticLinkageKey::Index { opd_tys: opd_uids } => LinkageKey::Index {
    //             opd_uids: opd_uids
    //                 .iter()
    //                 .map(|opd_uid| entity_uid(db, opd_uid))
    //                 .collect(),
    //         },
    //         __StaticLinkageKey::StructField {
    //             this_ty,
    //             field_ident,
    //             ..
    //         } => LinkageKey::StructField {
    //             this_ty_uid: entity_uid(db, this_ty),
    //             field_ident: db.custom_ident(field_ident),
    //         },
    //         __StaticLinkageKey::FeatureEagerBlock { route } => LinkageKey::FeatureEagerBlock {
    //             uid: entity_uid(db, route),
    //         },
    //     }
    // }

    // pub fn into_form(&self, db: &dyn EntityDefnQueryGroup) -> LinkageForm {
    //     match self {
    //         LinkageKey::VecConstructor { element_ty_uid } => LinkageForm::VecConstructor {
    //             element_ty: db.entity_route_by_uid(*element_ty_uid),
    //         },
    //         LinkageKey::TypeCall { ty_uid } => LinkageForm::TypeCall {
    //             ty: db.entity_route_by_uid(*ty_uid),
    //         },
    //         LinkageKey::Routine { routine_uid } => LinkageForm::Routine {
    //             routine: db.entity_route_by_uid(*routine_uid),
    //         },
    //         LinkageKey::Index { opd_uids } => LinkageForm::Index {
    //             opd_tys: opd_uids
    //                 .iter()
    //                 .map(|uid| db.entity_route_by_uid(*uid))
    //                 .collect(),
    //         },
    //         LinkageKey::StructField {
    //             this_ty_uid,
    //             field_ident,
    //         } => LinkageForm::StructFieldAccess {
    //             this_ty: db.entity_route_by_uid(*this_ty_uid),
    //             field_ident: *field_ident,
    //         },
    //         LinkageKey::FeatureEagerBlock { .. } => todo!(),
    //     }
    // }
}

fn entity_uid(_db: &dyn ResolveLinkage, _text: &str) -> EntityUid {
    todo!()
    // db.entity_uid(db.parse_route_from_text(text))
}
