mod index;
mod set;

use crate::helpers::DecTermFamily;

pub use self::index::*;
pub use self::set::*;

use super::*;
use husky_entity_tree::SynNodeRegionPath;
use salsa::DisplayWithDb;
use thiserror::Error;
use vec_like::VecSet;

/// svar are variables defined in a stack-like top-down manner through generics
#[salsa::interned]
pub struct DecSymbolicVariable {
    pub toolchain: Toolchain,
    pub ty: DecTermSymbolicVariableTypeResult<DecTerm>,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    /// todo: change to RefinedGenericIndex
    pub index: DecSymbolicVariableIndex,
}

impl DecSymbolicVariable {
    #[inline(always)]
    pub fn new_self_ty(
        db: &::salsa::Db,
        toolchain: Toolchain,
        registry: &mut DecSymbolicVariableRegistry,
    ) -> Self {
        // todo: general universe??? or ignore universes totally
        DecSymbolicVariable::new(
            db,
            toolchain,
            Ok(DecTerm::TYPE),
            registry.issue_self_ty_index(),
        )
    }

    #[inline(always)]
    pub fn new_self_value(
        db: &::salsa::Db,
        toolchain: Toolchain,
        registry: &mut DecSymbolicVariableRegistry,
        _self_ty_term: DecTerm,
    ) -> Self {
        // todo: general universe??? or ignore universes totally
        DecSymbolicVariable::new(
            db,
            toolchain,
            Ok(DecTerm::TYPE),
            registry.issue_self_value_index(),
        )
    }

    #[inline(always)]
    pub fn new_lifetime(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DecTermMenu,
        registry: &mut DecSymbolicVariableRegistry,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> (DecTermSymbolicVariableTypeResult<DecTerm>, Self) {
        let ty = Ok(menu.lifetime_ty());
        (
            ty,
            Self::new(
                db,
                toolchain,
                ty,
                registry.issue_explicit_lifetime_index(attrs, variance),
            ),
        )
    }

    #[inline(always)]
    pub fn new_place(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DecTermMenu,
        registry: &mut DecSymbolicVariableRegistry,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> (DecTermSymbolicVariableTypeResult<DecTerm>, Self) {
        let ty = Ok(menu.place_ty());
        (
            ty,
            Self::new(
                db,
                toolchain,
                ty,
                registry.issue_explicit_place_index(attrs, variance),
            ),
        )
    }

    #[inline(always)]
    pub fn new_ty(
        db: &::salsa::Db,
        toolchain: Toolchain,
        menu: &DecTermMenu,
        registry: &mut DecSymbolicVariableRegistry,
        attrs: DeclarativeTemplateVariableAttrs,
        variance: Option<Variance>,
    ) -> (DecTermSymbolicVariableTypeResult<DecTerm>, Self) {
        let ty = Ok(menu.ty0().into());
        (
            ty,
            DecSymbolicVariable::new(db, toolchain, ty, registry.issue_ty_index(attrs, variance)),
        )
    }

    pub fn new_const(
        db: &::salsa::Db,
        toolchain: Toolchain,
        attrs: DeclarativeTemplateVariableAttrs,
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        registry: &mut DecSymbolicVariableRegistry,
    ) -> Self {
        let idx = match ty {
            Ok(ty) => match ty.family(db) {
                DecTermFamily::Category(_) => todo!(),
                DecTermFamily::TypePath(ty_path) => {
                    registry.issue_const_path_leading_index(attrs, ty_path)
                }
                DecTermFamily::Other => registry.issue_const_other_index(attrs),
            },
            Err(_) => registry.issue_const_err_index(attrs),
        };
        Self::new(db, toolchain, ty, idx)
    }

    /// ephem is short for `ephemeral`
    pub fn new_ephem(
        db: &::salsa::Db,
        toolchain: Toolchain,
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        registry: &mut DecSymbolicVariableRegistry,
    ) -> Self {
        let idx = match ty {
            Ok(ty) => match ty.family(db) {
                DecTermFamily::Category(_) => todo!(),
                DecTermFamily::TypePath(ty_path) => {
                    registry.issue_ephem_path_leading_index(ty_path)
                }
                DecTermFamily::Other => registry.issue_ephem_other_index(),
            },
            Err(_) => todo!(),
        };
        Self::new(db, toolchain, ty, idx)
    }

    pub unsafe fn new_ad_hoc(
        db: &::salsa::Db,
        toolchain: Toolchain,
        ty: DecTerm,
        disambiguator: u8,
    ) -> Self {
        Self::new(
            db,
            toolchain,
            Ok(ty),
            DecSymbolicVariableIndex::new_ad_hoc(disambiguator),
        )
    }

    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        name_map: &DecSymbolicVariableNameMap,
    ) -> std::fmt::Result {
        name_map[self].display_fmt_with_db(f, db)
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecTermSymbolicVariableTypeErrorKind {
    #[error("signature declarative_term error")]
    SignatureDecTermError,
    #[error("sketch declarative_term error")]
    SketchDecTermError,
    #[error("cannot infer type expression term")]
    CannotInferTypeExprTerm(SynNodeRegionPath),
}

pub type DecTermSymbolicVariableTypeResult<T> = Result<T, DecTermSymbolicVariableTypeErrorKind>;

impl salsa::DisplayWithDb for DecSymbolicVariable {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // ad hoc
        f.write_fmt(format_args!("${:?}", self.index(db)))
    }
}
