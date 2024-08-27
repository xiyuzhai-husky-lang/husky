use crate::*;
use husky_dec_signature::helpers::projs::dec_var_full_projs;
use husky_devsoul::helpers::{
    DevsoulAnchor, DevsoulChart, DevsoulChartDim0, DevsoulChartDim1, DevsoulJointPedestal,
    DevsoulOrderedStaticVarMap, DevsoulPedestal, DevsoulStaticVarMap,
};
use husky_ki_repr::repr::KiDomainRepr;
use husky_linket_impl::pedestal::JointPedestal;
use husky_trace_protocol::chart::{ChartDim0, ChartDim1};
use husky_trace_protocol::{anchor::Anchor, chart::Chart};
use smallvec::SmallVec;
use vec_like::SmallVecSet;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn with_static_var_anchors<R>(
        &self,
        static_vars: impl IntoIterator<Item = (ItemPath, DevsoulAnchor<Devsoul>)>,
        f: impl FnMut(&Self, &DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChart<Devsoul, R>> {
        let db = self.db();
        let mut locked: SmallVecSet<ItemPathIdInterface, 2> = Default::default();
        let static_vars: SmallVec<
            [(
                ItemPath,
                DevsoulAnchor<Devsoul>,
                SmallVecSet<ItemPathIdInterface, 2>,
            ); 2],
        > = static_vars
            .into_iter()
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
        mut static_var_map: DevsoulOrderedStaticVarMap<Devsoul>,
        remaining_static_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut(&Self, &DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChartDim0<Devsoul, R>> {
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
        let joint_pedestal = JointPedestal::new(static_var_map);
        let r = f(self, &joint_pedestal)?;
        Some((joint_pedestal, r))
    }

    pub fn with_static_vars_aux1<R>(
        &self,
        mut static_var_map: DevsoulOrderedStaticVarMap<Devsoul>,
        remaining_static_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut(&Self, &DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChartDim1<Devsoul, R>> {
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
                                    &mut f,
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
