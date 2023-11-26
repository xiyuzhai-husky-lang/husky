mod val_domain_repr_guard;

pub(crate) use self::val_domain_repr_guard::ValDomainReprGuard;

use crate::*;
use husky_coword::Ident;
use husky_entity_path::FugitivePath;
use husky_val::{Val, ValArgument, ValDomain, ValOpn};
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = ValReprDb, jar = ValReprJar)]
pub struct ValRepr {
    pub val_domain_repr: ValDomainRepr,
    pub opn: ValOpn,
    #[return_ref]
    pub arguments: SmallVec<[ValArgumentRepr; 4]>,
    pub caching_class: ValCachingClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValArgumentRepr {
    Ordinary(ValRepr),
    Keyed(Ident, Option<ValRepr>),
    Variadic(Vec<ValRepr>),
    Branch {
        condition: Option<ValRepr>,
        stmts: SmallVec<[ValRepr; 4]>,
    },
}

impl ValRepr {
    pub fn new_val_item(path: FugitivePath, db: &dyn ValReprDb) -> Self {
        val_item_val_repr(db, path)
    }

    pub(crate) fn with_caching_class(
        self,
        caching_class: ValCachingClass,
        db: &dyn ValReprDb,
    ) -> Self {
        Self::new(
            db,
            self.val_domain_repr(db),
            self.opn(db),
            self.arguments(db).clone(),
            caching_class,
        )
    }
}

// #[salsa::tracked(jar = ValReprJar)]
fn val_item_val_repr(db: &dyn ValReprDb, path: FugitivePath) -> ValRepr {
    let domain = ValDomainRepr::Omni;
    let opr = ValOpn::ValItem(path);
    let opds = smallvec![];
    let caching_class = ValCachingClass::ValItem;
    ValRepr::new(db, domain, opr, opds, caching_class)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomainRepr {
    /// everything
    Omni,
    /// those where the val repr of type bool is defined and equals true
    ConditionSatisfied(ValRepr),
    /// those where the val repr of type bool is defined and equals false
    ConditionNotSatisfied(ValRepr),
    /// those where the val repr of type ControlFlow<(), _> is defined and equals Continue(())
    StmtNotReturned(ValRepr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ValCachingClass {
    ValItem,
    Variable,
    Expr,
    Stmt,
}

impl ValRepr {
    pub fn val(self, db: &dyn ValReprDb) -> Val {
        val_repr_val(db, self)
    }
}

#[salsa::tracked(jar = ValReprJar)]
fn val_repr_val(db: &dyn ValReprDb, val_repr: ValRepr) -> Val {
    Val::new(
        db,
        val_repr.val_domain_repr(db).val(db),
        val_repr.opn(db),
        val_repr
            .arguments(db)
            .iter()
            .map(|val_repr| val_repr.val_argument(db))
            .collect(),
    )
}

impl ValArgumentRepr {
    fn val_argument(&self, db: &dyn ValReprDb) -> ValArgument {
        match *self {
            ValArgumentRepr::Ordinary(val_repr) => ValArgument::Ordinary(val_repr.val(db)),
            ValArgumentRepr::Keyed(ident, val_repr) => {
                ValArgument::Keyed(ident, val_repr.map(|val_repr| val_repr.val(db)))
            }
            ValArgumentRepr::Variadic(ref val_reprs) => {
                ValArgument::Variadic(val_reprs.iter().map(|val_repr| val_repr.val(db)).collect())
            }
            ValArgumentRepr::Branch {
                condition,
                ref stmts,
            } => ValArgument::Branch {
                condition: condition.map(|condition| condition.val(db)),
                stmts: stmts.iter().map(|&stmt| stmt.val(db)).collect(),
            },
        }
    }
}

impl ValDomainRepr {
    pub fn val(self, db: &dyn ValReprDb) -> ValDomain {
        match self {
            ValDomainRepr::Omni => ValDomain::Omni,
            ValDomainRepr::ConditionSatisfied(val_repr) => {
                ValDomain::ConditionSatisfied(val_repr.val(db))
            }
            ValDomainRepr::ConditionNotSatisfied(val_repr) => {
                ValDomain::ConditionNotSatisfied(val_repr.val(db))
            }
            ValDomainRepr::StmtNotReturned(val_repr) => {
                ValDomain::StmtNotReturned(val_repr.val(db))
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn val_item_val_reprs(
    db: &::salsa::Db
    module_path: ModulePath,
) -> Vec<(FugitivePath, ValRepr)> {
    use husky_entity_kind::FugitiveKind;
    use husky_entity_path::{ItemPath, MajorItemPath};
    use husky_entity_syn_tree::helpers::paths::module_item_paths;

    module_item_paths(db, module_path)
        .iter()
        .filter_map(|&path| match path {
            ItemPath::MajorItem(MajorItemPath::Fugitive(path)) => match path.fugitive_kind(db) {
                FugitiveKind::Val => Some((path, ValRepr::new_val_item(path, db))),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

#[test]
fn val_item_val_repr_works() {
    let _db = DB::default();
    DB::default().ast_expect_test_debug_with_db(
        val_item_val_reprs,
        &AstTestConfig::new("val_item_val_reprs"),
    )
}
