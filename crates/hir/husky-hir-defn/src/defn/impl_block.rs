use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
#[salsa::debug_with_db(db = HirDefnDb)]
pub enum ImplBlockHirDefn {
    Type(TypeImplBlockHirDefn),
    TraitForType(TraitForTypeImplBlockHirDefn),
}

impl ImplBlockHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> ImplBlockPath {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.path(db).into(),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub fn hir_decl(self) -> ImplBlockHirDecl {
        match self {
            ImplBlockHirDefn::Type(hir_defn) => hir_defn.hir_decl().into(),
            ImplBlockHirDefn::TraitForType(hir_defn) => hir_defn.hir_decl().into(),
        }
    }
}

impl HasHirDefn for ImplBlockPath {
    type HirDefn = ImplBlockHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(match self.hir_decl(db)? {
            ImplBlockHirDecl::Type(hir_decl) => TypeImplBlockHirDefn { hir_decl }.into(),
            ImplBlockHirDecl::TraitForType(hir_decl) => {
                TraitForTypeImplBlockHirDefn { hir_decl }.into()
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
pub struct TypeImplBlockHirDefn {
    hir_decl: TypeImplBlockHirDecl,
}

impl TypeImplBlockHirDefn {
    pub fn hir_decl(self) -> TypeImplBlockHirDecl {
        self.hir_decl
    }

    pub fn path(self, db: &dyn HirDefnDb) -> TypeImplBlockPath {
        self.hir_decl.path(db)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
pub struct TraitForTypeImplBlockHirDefn {
    hir_decl: TraitForTypeImplBlockHirDecl,
}

impl TraitForTypeImplBlockHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> TraitForTypeImplBlockPath {
        self.hir_decl.path(db)
    }

    pub fn hir_decl(self) -> TraitForTypeImplBlockHirDecl {
        self.hir_decl
    }
}
