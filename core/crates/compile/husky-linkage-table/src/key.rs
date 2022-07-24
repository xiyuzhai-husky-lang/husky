use crate::*;
use __husky::init::__StaticLinkageKey;
use husky_entity_syntax::EntitySyntaxQueryGroup;
use smallvec::SmallVec;
use upcast::Upcast;

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
            __StaticLinkageKey::Index { opd_tys: opd_uids } => LinkageKey::ElementAccess {
                opd_uids: opd_uids
                    .iter()
                    .map(|opd_uid| entity_uid(db, opd_uid))
                    .collect(),
            },
            __StaticLinkageKey::StructEagerField {
                this_ty,
                field_ident,
                ..
            } => LinkageKey::StructFieldAccess {
                this_ty_uid: entity_uid(db, this_ty),
                field_ident: db.custom_ident(field_ident),
            },
            __StaticLinkageKey::FeatureEagerBlock { route } => LinkageKey::FeatureEagerBlock {
                uid: entity_uid(db, route),
            },
        }
    }

    pub fn into_form(&self, db: &dyn EntityDefnQueryGroup) -> LinkageForm {
        match self {
            LinkageKey::VecConstructor { element_ty_uid } => LinkageForm::VecConstructor {
                element_ty: db.entity_route_by_uid(*element_ty_uid),
            },
            LinkageKey::TypeCall { ty_uid } => LinkageForm::TypeCall {
                ty: db.entity_route_by_uid(*ty_uid),
            },
            LinkageKey::Routine { routine_uid } => LinkageForm::Routine {
                routine: db.entity_route_by_uid(*routine_uid),
            },
            LinkageKey::ElementAccess { opd_uids } => LinkageForm::ElementAccess {
                opd_tys: opd_uids
                    .iter()
                    .map(|uid| db.entity_route_by_uid(*uid))
                    .collect(),
            },
            LinkageKey::StructFieldAccess {
                this_ty_uid,
                field_ident,
            } => LinkageForm::StructFieldAccess {
                this_ty: db.entity_route_by_uid(*this_ty_uid),
                field_ident: *field_ident,
            },
            LinkageKey::FeatureEagerBlock { uid: feature_uid } => todo!(),
        }
    }
}

fn entity_uid(db: &dyn ResolveLinkage, text: &str) -> EntityUid {
    db.entity_uid(db.parse_route_from_text(text))
}
