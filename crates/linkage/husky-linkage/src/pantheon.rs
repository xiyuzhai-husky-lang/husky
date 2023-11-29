use crate::*;
use fxhash::FxHashMap;
use husky_manifest::HasPackageManifest;
use husky_vfs::PackagePath;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct LinkagePantheon {
    package_path: PackagePath,
    // map each linkage to a package where it's instantiated
    instantiation_map: FxHashMap<Linkage, PackagePath>,
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub(crate) fn valkyrie_linkage_pantheon(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> LinkagePantheon {
    let mut pantheon = LinkagePantheon {
        package_path,
        instantiation_map: Default::default(),
    };
    for dep in package_path
        .package_dependencies(db)
        .expect("no error at this stage")
    {
        pantheon.merge(valkyrie_linkage_pantheon(db, dep.package_path()))
    }
    // todo: add new linkages instantiated here
    pantheon
}

#[test]
fn linkage_pantheon_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, package_path| valkyrie_linkage_pantheon(db, package_path),
        &AstTestConfig::new("linkage_pantheon"),
    )
}

impl LinkagePantheon {
    pub fn new_linkages<'a>(&'a self) -> impl Iterator<Item = Linkage> + 'a {
        self.instantiation_map
            .iter()
            .filter_map(|(&linkage, &package_path)| {
                (package_path == self.package_path).then_some(linkage)
            })
    }

    fn merge(&mut self, other: &Self) {
        for (&linkage, &package_path) in other.instantiation_map.iter() {
            self.instantiation_map
                .entry(linkage)
                .or_insert(package_path);
        }
    }
}
