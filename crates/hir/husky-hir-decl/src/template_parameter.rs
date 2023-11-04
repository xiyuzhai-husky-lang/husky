use crate::*;
use husky_hir_ty::trai::HirTrait;
use husky_syn_expr::SynTemplateParameterSyndicate;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub struct HirTemplateParameter {
    data: HirTemplateParameterData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub enum HirTemplateParameterData {
    Type { ident: Ident, traits: Vec<HirTrait> },
    Constant { ident: Ident, ty: HirType },
    Lifetime { label: Label },
    Place { label: Label },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HirTemplateParameters(SmallVec<[HirTemplateParameter; 2]>);

impl HirTemplateParameters {
    pub fn from_syn(obelisks: &[SynTemplateParameterSyndicate], db: &dyn HirDeclDb) -> Self {
        todo!()
    }
}

impl std::ops::Deref for HirTemplateParameters {
    type Target = [HirTemplateParameter];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
