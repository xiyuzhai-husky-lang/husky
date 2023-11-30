use crate::{amazon::package_amazon_linkages, valkyrie::linkage_valkyrie_linkages, *};
use fxhash::FxHashMap;
use husky_manifest::HasPackageManifest;
use husky_vfs::PackagePath;
use vec_like::VecSet;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct LinkagePantheon {
    package_path: PackagePath,
    // map each linkage to a package where it's instantiated
    instantiation_map: FxHashMap<Linkage, PackagePath>,
    new_linkages: Vec<Linkage>,
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub(crate) fn package_valkyrie_linkage_pantheon(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> LinkagePantheon {
    let mut pantheon = LinkagePantheon {
        package_path,
        instantiation_map: Default::default(),
        new_linkages: Default::default(),
    };
    for dep in package_path
        .package_dependencies(db)
        .expect("no error at this stage")
    {
        pantheon.add_valkyrie_linkages_instantiated_by_package(package_valkyrie_linkage_pantheon(
            db,
            dep.package_path(),
        ))
    }
    let mut bar = 0;
    for &linkage in package_amazon_linkages(db, package_path) {
        pantheon.try_add_valkyrie_linkages_instantiated_by_linkage(linkage, db)
    }
    while bar < pantheon.new_linkages.len() {
        let new_bar = pantheon.new_linkages.len();
        for i in bar..pantheon.new_linkages.len() {
            pantheon.try_add_valkyrie_linkages_instantiated_by_linkage(pantheon.new_linkages[i], db)
        }
        bar = new_bar
    }
    pantheon
}

impl LinkagePantheon {
    pub fn new_linkages<'a>(&'a self) -> impl Iterator<Item = Linkage> + 'a {
        self.instantiation_map
            .iter()
            .filter_map(|(&linkage, &package_path)| {
                (package_path == self.package_path).then_some(linkage)
            })
    }

    fn add_valkyrie_linkages_instantiated_by_package(
        &mut self,
        other_package_linkage_pantheon: &Self,
    ) {
        for (&linkage, &package_path) in other_package_linkage_pantheon.instantiation_map.iter() {
            self.instantiation_map
                .entry(linkage)
                .or_insert(package_path);
        }
    }

    // do nothing if already instantiated
    fn try_add_valkyrie_linkages_instantiated_by_linkage(
        &mut self,
        linkage: Linkage,
        db: &::salsa::Db,
    ) {
        for &valkyrie_linkage in linkage_valkyrie_linkages(db, linkage) {
            match self.instantiation_map.get(&valkyrie_linkage) {
                Some(_) => (),
                None => self.add_new_linkage(valkyrie_linkage),
            }
        }
    }

    fn add_new_linkage(&mut self, linkage: Linkage) {
        self.instantiation_map.insert(linkage, self.package_path);
        self.new_linkages.push(linkage)
    }
}

#[test]
fn package_linkage_pantheon_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, package_path| package_valkyrie_linkage_pantheon(db, package_path),
        &AstTestConfig::new("package_linkage_pantheon"),
    )
}
