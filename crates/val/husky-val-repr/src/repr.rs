use crate::*;
use husky_entity_path::{EntityPathDb, FugitivePath};
use husky_val::{Val, ValDomain, ValOpr};
use smallvec::{smallvec, SmallVec};

#[salsa::interned(db = ValReprDb, jar = ValReprJar, override_debug)]
pub struct ValRepr {
    pub domain_repr: ValDomainRepr,
    pub opr: ValOpr,
    #[return_ref]
    pub opds: SmallVec<[ValRepr; 2]>,
    pub caching_strategy: ValReprCachingStrategy,
}

impl<_Db: ValReprDb + ?Sized> ::salsa::DebugWithDb<_Db> for ValRepr {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &_Db,
        _level: salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        let _db = <_Db as ::salsa::DbWithJar<ValReprJar>>::as_jar_db(_db);
        let mut debug_struct = &mut f.debug_struct("Val");
        if _level.is_root() {
            debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        }
        debug_struct = debug_struct.field(
            "domain_repr",
            &::salsa::debug::helper::SalsaDebug::<
                ValDomain,
                <ValReprJar as salsa::jar::Jar<'_>>::DynDb,
            >::salsa_debug(
                #[allow(clippy::needless_borrow)]
                &self.domain_repr(_db),
                _db,
                _level.next(),
            ),
        );
        debug_struct =
            debug_struct.field(
                "opr",
                &::salsa::debug::helper::SalsaDebug::<
                    ValOpr,
                    <ValReprJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &self.opr(_db),
                    _db,
                    _level.next(),
                ),
            );
        debug_struct = debug_struct.field("opds", &self.opds(_db));
        debug_struct.finish()
    }
}

impl ValRepr {
    pub(crate) fn new_val_item(path: FugitivePath, db: &dyn ValReprDb) -> Self {
        let domain = ValDomainRepr::Omni;
        let opr = ValOpr::Fugitive(path);
        let opds = smallvec![];
        let caching_strategy = ValReprCachingStrategy::Cache;
        Self::new(db, domain, opr, opds, caching_strategy)
    }
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
pub enum ValReprCachingStrategy {
    Cache,
    Skip,
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
        val_repr.domain_repr(db).val(db),
        val_repr.opr(db),
        val_repr
            .opds(db)
            .iter()
            .map(|val_repr| val_repr.val(db))
            .collect(),
    )
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
pub(crate) fn val_item_val_reprs(db: &DB, module_path: ModulePath) -> Vec<(FugitivePath, ValRepr)> {
    use husky_entity_kind::FugitiveKind;
    use husky_entity_path::{ItemPath, MajorItemPath};
    use husky_entity_syn_tree::helpers::paths::module_item_paths;
    use husky_hir_defn::HasHirDefn;

    module_item_paths(db, module_path)
        .as_ref()
        .expect("all modules should be guaranteed to be valid")
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
    // todo: why compiler needs this line to work?
    use husky_ast::test_utils::AstTestUtils;
    let db = DB::default();
    DB::default().ast_expect_test_debug_with_db(
        val_item_val_reprs,
        &AstTestConfig::new("val_item_val_reprs"),
    )
}
