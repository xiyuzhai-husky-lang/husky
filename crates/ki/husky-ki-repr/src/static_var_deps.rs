use husky_entity_path::path::ItemPath;
use husky_ki::{Ki, KiOpn};
use husky_sem_static_var_deps::static_var_deps::SemStaticVarDeps;
use vec_like::OrderedSmallVecSet;

#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KiStaticVarDeps(OrderedSmallVecSet<ItemPath, 4>);

// ad hoc, maybe some other consideration about template arguments?
impl From<&SemStaticVarDeps> for KiStaticVarDeps {
    fn from(value: &SemStaticVarDeps) -> Self {
        Self((**value).clone())
    }
}

impl KiStaticVarDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        self.0.extend(&other.0);
    }

    pub(crate) fn insert(&mut self, item_path: ItemPath) {
        self.0.insert(item_path);
    }
}

pub trait HasKiStaticVarDeps: Copy {
    fn ki_static_var_deps(self, db: &::salsa::Db) -> &KiStaticVarDeps;
}

impl HasKiStaticVarDeps for Ki {
    fn ki_static_var_deps(self, db: &salsa::Db) -> &KiStaticVarDeps {
        ki_ki_static_var_deps(db, self)
    }
}

// ad hoc
#[salsa::tracked(return_ref)]
fn ki_ki_static_var_deps(db: &::salsa::Db, ki: Ki) -> KiStaticVarDeps {
    match ki.opn(db) {
        KiOpn::Return | KiOpn::Require | KiOpn::Assert => todo!(),
        KiOpn::ValItemLazilyDefined(_) => todo!(),
        KiOpn::FunctionRitchie(_) => todo!(),
        KiOpn::Prefix(_) => todo!(),
        KiOpn::Suffix(_) => todo!(),
        KiOpn::Binary(_) => todo!(),
        KiOpn::Linkage(_) => todo!(),
        KiOpn::EvalDiscarded => todo!(),
        KiOpn::Literal(_) => todo!(),
        KiOpn::Branches => todo!(),
        KiOpn::TypeVariant(_) => todo!(),
        KiOpn::Be { pattern_data } => todo!(),
        KiOpn::Unwrap {} => todo!(),
        KiOpn::Index => todo!(),
    }
}
