use crate::*;
use husky_dec_signature::helpers::projs::dec_var_full_projs;
use husky_devsoul::helpers::{DevsoulAnchor, DevsoulStaticVarMap};
use husky_devsoul_interface::{
    anchor::Anchor,
    chart::{Chart, ChartDim0, ChartDim1},
};
use husky_ki_repr::repr::KiDomainRepr;
use smallvec::SmallVec;
use vec_like::SmallVecSet;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn with_static_vars<R>(
        &self,
        static_vars: impl Iterator<Item = (ItemPath, DevsoulAnchor<Devsoul>)>,
        f: impl FnMut() -> Option<R>,
    ) -> Option<Chart<DevsoulStaticVarId<Devsoul>, R>> {
        let db = self.db();
        let mut locked: SmallVecSet<ItemPathIdInterface, 2> = Default::default();
        let static_vars: SmallVec<
            [(
                ItemPath,
                DevsoulAnchor<Devsoul>,
                SmallVecSet<ItemPathIdInterface, 2>,
            ); 2],
        > = static_vars
            .filter_map(
                |(path, anchor)| -> Option<(
                    ItemPath,
                    DevsoulAnchor<Devsoul>,
                    SmallVecSet<ItemPathIdInterface, 2>,
                )> {
                    let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
                        todo!()
                    };
                    match path.kind(db) {
                        MajorFormKind::TypeVar
                        | MajorFormKind::StaticMut
                        | MajorFormKind::Compterm
                        | MajorFormKind::Conceptual => return None,
                        MajorFormKind::StaticVar => (),
                        _ => unreachable!(),
                    }
                    let locked1 = locked.clone();
                    locked.extend(
                        dec_var_full_projs(db, path)
                            .as_ref()
                            .unwrap()
                            .iter()
                            .copied()
                            .map(|path| (*path).into()),
                    );
                    Some((path.into(), anchor, locked1))
                },
            )
            .collect();
        let number_of_generics = static_vars
            .iter()
            .filter(|&(path, anchor, ref locked)| anchor.is_generic())
            .count();
        match number_of_generics {
            0 => self
                .with_static_vars_aux0(Default::default(), &static_vars, f)
                .map(Into::into),
            1 => self
                .with_static_vars_aux1(Default::default(), &static_vars, f)
                .map(Into::into),
            2 => {
                todo!()
            }
            _ => todo!(),
        }
    }

    pub fn with_static_vars_aux0<R>(
        &self,
        mut static_var_map: DevsoulStaticVarMap<Devsoul>,
        remaining_static_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut() -> Option<R>,
    ) -> Option<ChartDim0<DevsoulStaticVarId<Devsoul>, R>> {
        let db = self.db();
        for &(path, anchor, ref locked) in remaining_static_vars {
            let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path else {
                todo!()
            };
            let linket_impl = self
                .comptime
                .linket_impl(Linket::new_static_var(major_form_path, db));
            let Anchor::Specific(static_var_id) = anchor else {
                unreachable!()
            };
            todo!()
        }
        Some((static_var_map, f()?))
    }

    pub fn with_static_vars_aux1<R>(
        &self,
        mut static_var_map: DevsoulStaticVarMap<Devsoul>,
        remaining_static_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut() -> Option<R>,
    ) -> Option<ChartDim1<DevsoulStaticVarId<Devsoul>, R>> {
        let &[(path, anchor, ref locked), ref remaining_static_vars @ ..] = remaining_static_vars
        else {
            unreachable!()
        };
        let db = self.db();
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path else {
            todo!()
        };
        let linket_impl = self
            .comptime
            .linket_impl(Linket::new_static_var(major_form_path, db));
        match anchor {
            Anchor::Specific(static_var_id) => {
                static_var_map.insert(((*path).into(), static_var_id));
                linket_impl
                    .with_static_var_id(static_var_id, locked, || {
                        self.with_static_vars_aux1(static_var_map, remaining_static_vars, f)
                    })
                    .ok()
                    .flatten()
            }
            Anchor::Generic { limit } => Some(
                linket_impl
                    .all_static_var_ids(locked)
                    .filter_map(|static_var_id| {
                        let mut static_var_map = static_var_map.clone();
                        static_var_map.insert(((*path).into(), static_var_id));
                        linket_impl
                            .with_static_var_id(static_var_id, locked, || {
                                self.with_static_vars_aux0(
                                    static_var_map,
                                    remaining_static_vars,
                                    || f(),
                                )
                            })
                            .ok()
                            .flatten()
                    })
                    .take(limit)
                    .collect(),
            ),
        }
    }
}
