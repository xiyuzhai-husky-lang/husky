mod domain_repr_guard;
pub mod source;

pub(crate) use self::domain_repr_guard::KiDomainReprGuard;

use self::source::*;
use crate::*;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_hir_defn::defn::{major_item::form::MajorFormHirDefn, HasHirDefn};
use husky_ki::{Ki, KiArgument, KiDomain, KiOpn, KiRuntimeConstant};
use husky_linkage::linkage::Linkage;
use husky_task_interface::ki_repr::{KiDomainReprInterface, KiReprInterface};
use smallvec::{smallvec, SmallVec};

#[salsa::tracked(db = KiReprDb, jar = KiReprJar, constructor = new_inner)]
pub struct KiRepr {
    pub ki_domain_repr: KiDomainRepr,
    pub opn: KiOpn,
    #[return_ref]
    pub arguments: SmallVec<[KiArgumentRepr; 4]>,
    /// the source tells the code and the dependent variables that generates this val
    pub source: KiReprSource,
    pub caching_class: KiCachingClass,
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
    use husky_task_interface::ki_repr::KiArgumentReprInterface;

    assert_eq!(
        std::mem::size_of::<KiArgumentRepr>(),
        std::mem::size_of::<KiArgumentReprInterface>()
    )
}

impl KiRepr {
    pub fn new(
        ki_domain_repr: KiDomainRepr,
        opn: KiOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        source: KiReprSource,
        db: &::salsa::Db,
    ) -> Self {
        Self::new_inner(
            db,
            ki_domain_repr,
            opn,
            arguments,
            source,
            source.caching_class(),
        )
    }

    pub fn new_val_item(path: MajorFormPath, db: &::salsa::Db) -> Self {
        val_item_ki_repr(db, path)
    }

    pub(crate) fn with_source(self, source: KiReprSource, db: &::salsa::Db) -> Self {
        Self::new(
            self.ki_domain_repr(db),
            self.opn(db),
            self.arguments(db).clone(),
            source,
            db,
        )
    }
}

#[salsa::tracked(jar = KiReprJar)]
fn val_item_ki_repr(db: &::salsa::Db, path: MajorFormPath) -> KiRepr {
    let domain = KiDomainRepr::Omni;
    let MajorFormHirDefn::Val(hir_defn) = path.hir_defn(db).unwrap() else {
        unreachable!()
    };
    let opn = match Linkage::new_val_item(path, db) {
        Some(linkage) => KiOpn::Linkage(linkage),
        None => KiOpn::ValItemLazilyDefined(path),
    };
    let opds = smallvec![];
    let caching_class = KiCachingClass::ValItem;
    KiRepr::new(domain, opn, opds, KiReprSource::ValItem(path), db)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KiDomainRepr {
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
fn ki_domain_repr_size_works() {
    assert_eq!(
        std::mem::size_of::<KiDomainRepr>(),
        std::mem::size_of::<KiDomainReprInterface>(),
    )
}

impl Into<KiDomainReprInterface> for KiDomainRepr {
    fn into(self) -> KiDomainReprInterface {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<KiDomainReprInterface> for KiDomainRepr {
    fn from(ki_domain_repr: KiDomainReprInterface) -> Self {
        unsafe { std::mem::transmute(ki_domain_repr) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KiCachingClass {
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
        ki_repr.ki_domain_repr(db).val(db),
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

impl KiDomainRepr {
    pub fn val(self, db: &::salsa::Db) -> KiDomain {
        match self {
            KiDomainRepr::Omni => KiDomain::Omni,
            KiDomainRepr::ConditionSatisfied(ki_repr) => {
                KiDomain::ConditionSatisfied(ki_repr.val(db))
            }
            KiDomainRepr::ConditionNotSatisfied(ki_repr) => {
                KiDomain::ConditionNotSatisfied(ki_repr.val(db))
            }
            KiDomainRepr::StmtNotReturned(ki_repr) => KiDomain::StmtNotReturned(ki_repr.val(db)),
            KiDomainRepr::ExprNotReturned(_) => todo!(),
        }
    }
}

#[cfg(test)]
pub(crate) fn val_item_ki_reprs(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<(MajorFormPath, KiRepr)> {
    use husky_entity_kind::MajorFormKind;
    use husky_entity_path::path::{major_item::MajorItemPath, ItemPath};
    use husky_entity_tree::helpers::paths::module_item_paths;

    module_item_paths(db, module_path)
        .iter()
        .filter_map(|&path| match path {
            ItemPath::MajorItem(MajorItemPath::Form(path)) => match path.kind(db) {
                MajorFormKind::Val => Some((path, KiRepr::new_val_item(path, db))),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

#[test]
fn val_item_ki_repr_works() {
    let _db = DB::default();
    DB::ast_rich_test_debug_with_db(
        val_item_ki_reprs,
        &AstTestConfig::new(
            "val_item_ki_reprs",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::VAL,
        ),
    )
}
