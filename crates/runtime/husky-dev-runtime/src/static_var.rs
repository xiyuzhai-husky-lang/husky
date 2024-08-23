use crate::*;
use husky_dec_signature::helpers::projs::dec_var_full_projs;
use husky_devsoul::helpers::DevsoulAnchor;
use smallvec::SmallVec;
use vec_like::SmallVecSet;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn with_static_vars<R>(
        &self,
        static_vars: impl Iterator<Item = (ItemPath, DevsoulAnchor<Devsoul>)>,
        f: impl FnOnce() -> R,
    ) -> impl Iterator<Item = ((), R)> {
        let db = self.db();
        let mut locked: SmallVecSet<ItemPath, 2> = Default::default();
        let static_vars: SmallVec<
            [(ItemPath, DevsoulAnchor<Devsoul>, SmallVecSet<ItemPath, 2>); 2],
        > = static_vars
            .filter_map(|(path, anchor)|->Option<(ItemPath, DevsoulAnchor<Devsoul>, SmallVecSet<ItemPath, 2>)> {
                let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
                    todo!()
                };
                match path.kind(db) {
                    MajorFormKind::Ritchie(_) => todo!(),
                    MajorFormKind::TypeAlias => todo!(),
                    MajorFormKind::TypeVar => return None,
                    MajorFormKind::Val => todo!(),
                    MajorFormKind::StaticMut => todo!(),
                    MajorFormKind::StaticVar => (),
                    MajorFormKind::Compterm => todo!(),
                    MajorFormKind::Conceptual => todo!(),
                }
                let locked1 = locked.clone();
                locked.extend(dec_var_full_projs(db,path).as_ref().unwrap().iter().copied().map(Into::into));
                Some((path.into(), anchor, locked1))
            })
            .collect();
        todo!();
        // let StandardLinketImpl::StaticVar {
        //             ids, replace_id, ..
        //         } = runtime
        //             .comptime
        //             .linket_impl(Linket::new_static_var(path, db))
        //         else {
        //             unreachable!()
        //         };
        //         for id in ids.zip(0..100) {
        //             set_up_for_testing(0)
        //         }
        [((), f())].into_iter()
    }
}
