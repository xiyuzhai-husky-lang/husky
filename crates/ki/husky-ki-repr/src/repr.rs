pub mod source;
mod val_domain_repr_guard;

pub(crate) use self::val_domain_repr_guard::ValDomainReprGuard;

use self::source::*;
use crate::*;
use husky_coword::Ident;
use husky_entity_path::FugitivePath;
use husky_hir_defn::{FugitiveHirDefn, HasHirDefn};
use husky_hir_expr::HirExprIdx;
use husky_ki::{Ki, KiArgument, KiRuntimeConstant, ValDomain, ValOpn};
use husky_linkage::linkage::Linkage;
use husky_task_interface::ki_repr::{
    KiArgumentReprInterface, KiReprInterface, ValDomainReprInterface,
};
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = KiReprDb, jar = KiReprJar, constructor = new_inner)]
pub struct KiRepr {
    pub val_domain_repr: ValDomainRepr,
    pub opn: ValOpn,
    #[return_ref]
    pub arguments: SmallVec<[KiArgumentRepr; 4]>,
    /// the source tells the code and the dependent variables that generates this val
    pub source: KiReprSource,
    pub caching_class: ValCachingClass,
}

impl Into<KiReprInterface> for KiRepr {
    fn into(self) -> KiReprInterface {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<KiReprInterface> for KiRepr {
    fn from(ki_repr: KiReprInterface) -> Self {
        unsafe { std::mem::transmute(ki_repr) }
    }
}

#[test]
fn ki_repr_size_works() {
    assert_eq!(
        std::mem::size_of::<KiRepr>(),
        std::mem::size_of::<KiReprInterface>()
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KiArgumentRepr {
    Simple(KiRepr),
    Keyed(Option<KiRepr>),
    Variadic(SmallVec<[KiRepr; 4]>),
    Branch {
        condition: Option<KiRepr>,
        stmts: SmallVec<[KiRepr; 4]>,
    },
    // `None` means no runtime constants
    RuntimeConstants(SmallVec<[KiRuntimeConstant; 4]>),
}

#[test]
fn val_argument_repr_size_works() {
    assert_eq!(
        std::mem::size_of::<KiArgumentRepr>(),
        std::mem::size_of::<KiArgumentReprInterface>()
    )
}

impl KiRepr {
    pub fn new(
        val_domain_repr: ValDomainRepr,
        opn: ValOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        source: KiReprSource,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(
            db,
            val_domain_repr,
            opn,
            arguments,
            source,
            source.caching_class(),
        )
    }

    pub fn new_val_item(path: FugitivePath, db: &::salsa::Db) -> Self {
        val_item_ki_repr(db, path)
    }

    pub(crate) fn with_source(self, source: KiReprSource, db: &::salsa::Db) -> Self {
        Self::new(
            self.val_domain_repr(db),
            self.opn(db),
            self.arguments(db).clone(),
            source,
            db,
        )
    }
}

#[salsa::tracked(jar = KiReprJar)]
fn val_item_ki_repr(db: &::salsa::Db, path: FugitivePath) -> KiRepr {
    let domain = ValDomainRepr::Omni;
    let FugitiveHirDefn::Ki(hir_defn) = path.hir_defn(db).unwrap() else {
        unreachable!()
    };
    let opn = match Linkage::new_val_item(path, db) {
        Some(linkage) => ValOpn::Linkage(linkage),
        None => ValOpn::ValItemLazilyDefined(path),
    };
    let opds = smallvec![];
    let caching_class = ValCachingClass::ValItem;
    KiRepr::new(domain, opn, opds, KiReprSource::ValItem(path), db)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValDomainRepr {
    /// everything
    Omni,
    /// those where the val repr of type bool is defined and equals true
    ConditionSatisfied(KiRepr),
    /// those where the val repr of type bool is defined and equals false
    ConditionNotSatisfied(KiRepr),
    /// those where the val repr of type ControlFlow<(), _> is defined and equals Continue(())
    StmtNotReturned(KiRepr),
    ExprNotReturned(KiRepr),
}

#[test]
fn val_domain_repr_size_works() {
    assert_eq!(
        std::mem::size_of::<ValDomainRepr>(),
        std::mem::size_of::<ValDomainReprInterface>(),
    )
}

impl Into<ValDomainReprInterface> for ValDomainRepr {
    fn into(self) -> ValDomainReprInterface {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<ValDomainReprInterface> for ValDomainRepr {
    fn from(val_domain_repr: ValDomainReprInterface) -> Self {
        unsafe { std::mem::transmute(val_domain_repr) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ValCachingClass {
    ValItem,
    Variable,
    Expr,
    Stmt,
    Condition,
    RuntimeConstant,
}

impl KiRepr {
    pub fn val(self, db: &::salsa::Db) -> Ki {
        ki_repr_ki(db, self)
    }
}

#[salsa::tracked(jar = KiReprJar)]
fn ki_repr_ki(db: &::salsa::Db, ki_repr: KiRepr) -> Ki {
    Ki::new(
        db,
        ki_repr.val_domain_repr(db).val(db),
        ki_repr.opn(db),
        ki_repr
            .arguments(db)
            .iter()
            .map(|ki_repr| ki_repr.val_argument(db))
            .collect(),
    )
}

impl KiArgumentRepr {
    fn val_argument(&self, db: &::salsa::Db) -> KiArgument {
        match *self {
            KiArgumentRepr::Simple(ki_repr) => KiArgument::Simple(ki_repr.val(db)),
            KiArgumentRepr::Keyed(ki_repr) => {
                KiArgument::Keyed(ki_repr.map(|ki_repr| ki_repr.val(db)))
            }
            KiArgumentRepr::Variadic(ref ki_reprs) => {
                KiArgument::Variadic(ki_reprs.iter().map(|ki_repr| ki_repr.val(db)).collect())
            }
            KiArgumentRepr::Branch {
                condition,
                ref stmts,
            } => KiArgument::Branch {
                condition: condition.map(|condition| condition.val(db)),
                stmts: stmts.iter().map(|&stmt| stmt.val(db)).collect(),
            },
            KiArgumentRepr::RuntimeConstants(ref ki_reprs) => {
                KiArgument::RuntimeConstants(ki_reprs.clone())
            }
        }
    }
}

impl ValDomainRepr {
    pub fn val(self, db: &::salsa::Db) -> ValDomain {
        match self {
            ValDomainRepr::Omni => ValDomain::Omni,
            ValDomainRepr::ConditionSatisfied(ki_repr) => {
                ValDomain::ConditionSatisfied(ki_repr.val(db))
            }
            ValDomainRepr::ConditionNotSatisfied(ki_repr) => {
                ValDomain::ConditionNotSatisfied(ki_repr.val(db))
            }
            ValDomainRepr::StmtNotReturned(ki_repr) => ValDomain::StmtNotReturned(ki_repr.val(db)),
            ValDomainRepr::ExprNotReturned(_) => todo!(),
        }
    }
}

#[cfg(test)]
pub(crate) fn val_item_ki_reprs(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(FugitivePath, KiRepr)> {
    use husky_entity_kind::MajorFugitiveKind;
    use husky_entity_path::{ItemPath, MajorItemPath};
    use husky_entity_tree::helpers::paths::module_item_paths;

    module_item_paths(db, module_path)
        .iter()
        .filter_map(|&path| match path {
            ItemPath::MajorItem(MajorItemPath::Fugitive(path)) => {
                match path.major_fugitive_kind(db) {
                    MajorFugitiveKind::Val => Some((path, KiRepr::new_val_item(path, db))),
                    _ => None,
                }
            }
            _ => None,
        })
        .collect()
}

#[test]
fn val_item_ki_repr_works() {
    let _db = DB::default();
    DB::ast_expect_test_debug_with_db(
        val_item_ki_reprs,
        &AstTestConfig::new(
            "val_item_ki_reprs",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::VAL,
        ),
    )
}
