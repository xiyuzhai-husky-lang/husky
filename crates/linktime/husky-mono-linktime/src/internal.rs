mod libraries;
mod linket_impls;

use self::linket_impls::generate_linket_impls;
use self::{libraries::MonoLinketLibraries, linket_impls::LinketImplMap};
use crate::*;
use husky_linket::version_stamp::LinketVersionStamp;
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use version_stamp::HasVersionStamp;

pub struct MonoLinktimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    target_path: LinktimeTargetPath,
    /// this is needed to kep Box<dyn StaticDyn> valid
    past_libraries: Vec<MonoLinketLibraries>,
    libraries: MonoLinketLibraries,
    linket_impls: LinketImplMap<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> MonoLinktimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        let Ok(libraries) = MonoLinketLibraries::generate(target_path, db) else {
            todo!("error in generating libraries")
        };
        let linket_impls = generate_linket_impls(target_path, &libraries, db);
        Self {
            target_path,
            past_libraries: vec![],
            libraries,
            linket_impls,
        }
    }

    pub(crate) fn get_linket_impl(&self, linket: Linket, db: &::salsa::Db) -> Option<LinketImpl> {
        let Some(&(version_stamp, linket_impl)) = self.linket_impls.get(&linket) else {
            use husky_print_utils::p;
            use salsa::DebugWithDb;
            let _linkets: Vec<Linket> = self.linket_impls.clone().into_keys().collect();
            // let old_linket: Linket = unsafe { std::mem::transmute(194u32) };
            // let LinketData::UnveilAssocFunctionRitchie {
            //     path: old_path,
            //     instantiation: old_instantiation,
            // } = old_linket.data(db)
            // else {
            //     unreachable!()
            // };
            // let LinketData::UnveilAssocFunctionRitchie {
            //     path,
            //     instantiation,
            // } = linket.data(db)
            // else {
            //     unreachable!()
            // };
            p!(linket.debug(db), linket);
            unreachable!()
        };
        (version_stamp == linket.version_stamp(db)).then_some(linket_impl)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linket_impl_with_reload(
        &mut self,
        key: Linket,
        db: &::salsa::Db,
    ) -> LinketImpl {
        let (deps, linket) = self
            .linket_impls
            .get(&key)
            .copied()
            .expect("should be some");
        if deps == key.version_stamp(db) {
            return linket;
        }
        self.reload(db);
        self.linket_impls
            .get(&key)
            .copied()
            .expect("should be some")
            .1
    }

    fn reload(&mut self, db: &::salsa::Db) {
        let libraries = std::mem::replace(
            &mut self.libraries,
            MonoLinketLibraries::generate(self.target_path, db).unwrap(),
        );
        self.past_libraries.push(libraries);
        self.linket_impls = generate_linket_impls(self.target_path, &self.libraries, db)
    }
}
