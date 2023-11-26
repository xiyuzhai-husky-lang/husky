mod associated_item;
mod major_item;

use crate::*;
use vec_like::VecSet;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct HirDefnVersionStamp {
    hir_defn: HirDefn,
    #[return_ref]
    item_hir_defns_in_current_crate: VecSet<HirDefn>,
    #[return_ref]
    item_hir_defn_version_stamps_in_other_local_crates: VecSet<HirDefnVersionStamp>,
}

impl HirDefnVersionStamp {
    pub(crate) fn new(hir_defn: impl Into<HirDefn>, db: &dyn HirDefnDb) -> Self {
        let mut builder = HirDefnVersionStampSimpleBuilder::new(hir_defn.into(), db);
        builder.collect_all();
        builder.finish()
    }
}

pub struct HirDefnVersionStampSimpleBuilder<'a> {
    hir_defn: HirDefn,
    item_hir_defns_in_current_crate: VecSet<HirDefn>,
    item_hir_defn_version_stamps_in_other_local_crates: VecSet<HirDefnVersionStamp>,
    db: &'a dyn HirDefnDb,
}

impl<'a> HirDefnVersionStampSimpleBuilder<'a> {
    fn new(hir_defn: HirDefn, db: &'a dyn HirDefnDb) -> Self {
        Self {
            hir_defn,
            item_hir_defns_in_current_crate: VecSet::new_one_elem_set(hir_defn),
            item_hir_defn_version_stamps_in_other_local_crates: Default::default(),
            db,
        }
    }

    fn collect_all(&mut self) {
        let mut round_start = 0;
        while round_start < self.item_hir_defns_in_current_crate.len() {
            let len = self.item_hir_defns_in_current_crate.len();
            self.collect_round(round_start);
            round_start = len
        }
    }

    fn collect_round(&mut self, round_start: usize) {
        let db = self.db;
        for i in round_start..self.item_hir_defns_in_current_crate.len() {
            let hir_defn = self.item_hir_defns_in_current_crate[i];
            let hir_defn_dependencies = hir_defn.dependencies(db).unwrap();
            for &item_path in hir_defn_dependencies.item_paths_in_current_crate(db).iter() {
                self.item_hir_defns_in_current_crate
                    .insert(item_path.hir_defn(db).unwrap())
            }
            for &item_path in hir_defn_dependencies
                .item_paths_in_other_local_crates(db)
                .iter()
            {
                self.item_hir_defn_version_stamps_in_other_local_crates
                    .insert(
                        item_path
                            .hir_defn(db)
                            .unwrap()
                            .opt_version_stamp(db)
                            .unwrap(),
                    )
            }
        }
    }

    fn finish(self) -> HirDefnVersionStamp {
        HirDefnVersionStamp::new_inner(
            self.db,
            self.hir_defn,
            self.item_hir_defns_in_current_crate,
            self.item_hir_defn_version_stamps_in_other_local_crates,
        )
    }
}

#[cfg(test)]
pub(crate) fn module_hir_defn_version_stamps(
    db: &::salsa::Db
    module_path: ModulePath,
) -> Vec<(ItemPath, Option<Option<HirDefnVersionStamp>>)> {
    use husky_entity_syn_tree::helpers::paths::module_item_paths;
    use salsa::test_utils::Db;
    module_item_paths(db, module_path)
        .iter()
        .map(|&path| {
            (
                path,
                path.hir_defn(db)
                    .map(|hir_defn| hir_defn.opt_version_stamp(db)),
            )
        })
        .collect()
}

#[test]
fn module_hir_defn_version_stamps_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug(
        module_hir_defn_version_stamps,
        &AstTestConfig::new("module_hir_defn_version_stamps"),
    )
}
