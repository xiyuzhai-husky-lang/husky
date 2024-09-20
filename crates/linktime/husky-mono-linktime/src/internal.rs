mod library;
mod linket_impls;

use self::linket_impls::generate_linket_impl_map;
use self::{library::MonoLinketsLibrary, linket_impls::LinketImplMap};
use crate::*;
use husky_linket::{linket::LinketData, version_stamp::LinketVersionStamp};
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;
use version_stamp::HasVersionStamp;

pub struct MonoLinktimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    target_path: LinktimeTargetPath,
    /// this is needed to kep Box<dyn StaticDyn> valid
    past_libraries: Vec<MonoLinketsLibrary<LinketImpl>>,
    current_library: MonoLinketsLibrary<LinketImpl>,
    linket_impl_map: LinketImplMap<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> MonoLinktimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn new(target_path: LinktimeTargetPath, db: &::salsa::Db) -> Self {
        let Ok(current_library) = MonoLinketsLibrary::new(target_path, db) else {
            panic!("error in generating libraries")
        };
        let linket_impl_map = generate_linket_impl_map(target_path, &current_library, db);
        Self {
            target_path,
            past_libraries: vec![],
            current_library,
            linket_impl_map,
        }
    }

    /// although nothing on this side is modified, we do have modification on the linket side.
    pub(crate) fn init(&mut self, runtime: &'static dyn IsDevRuntimeInterfaceDyn<LinketImpl>) {
        self.current_library.init(runtime);
    }
}

impl<LinketImpl: IsLinketImpl> MonoLinktimeInternal<LinketImpl>
where
    LinketImpl: IsLinketImpl,
{
    pub(crate) fn get_linket_impl(&self, linket: Linket, db: &::salsa::Db) -> Option<LinketImpl> {
        let Some(&(version_stamp, linket_impl)) = self.linket_impl_map.get(&linket) else {
            use husky_print_utils::p;
            use salsa::DebugWithDb;
            let linkets: Vec<Linket> = self.linket_impl_map.clone().into_keys().collect();
            let linkets_filtered: Vec<Linket> = linkets
                .iter()
                .copied()
                .filter(|linket| match linket.data(db) {
                    LinketData::EnumVariantConstructor {
                        self_ty,
                        path,
                        instantiation,
                    } if path.ident(db).data(db) == "Unknown" => true,
                    _ => false,
                })
                .collect();
            {
                use similar::ChangeTag;
                let l0 = format!("{:#?}", linkets_filtered[0].data(db).debug(db));
                let l1 = format!("{:#?}", linket.data(db).debug(db));
                let diff = similar::TextDiff::from_lines(&l0, &l1);
                println!("-----------------------------------------------");
                for change in diff.iter_all_changes() {
                    let sign = match change.tag() {
                        ChangeTag::Delete => "-",
                        ChangeTag::Insert => "+",
                        ChangeTag::Equal => " ",
                    };
                    print!("{}{}", sign, change);
                }
                println!("-----------------------------------------------");
            }
            p!(
                linkets_filtered.debug(db),
                linkets_filtered[0] == linket,
                linket.data(db).debug(db),
                linket
            );
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
            .linket_impl_map
            .get(&key)
            .copied()
            .expect("should be some");
        if deps == key.version_stamp(db) {
            return linket;
        }
        self.reload(db);
        self.linket_impl_map
            .get(&key)
            .copied()
            .expect("should be some")
            .1
    }

    fn reload(&mut self, db: &::salsa::Db) {
        let libraries = std::mem::replace(
            &mut self.current_library,
            MonoLinketsLibrary::new(self.target_path, db).unwrap(),
        );
        self.past_libraries.push(libraries);
        self.linket_impl_map = generate_linket_impl_map(self.target_path, &self.current_library, db)
    }
}
