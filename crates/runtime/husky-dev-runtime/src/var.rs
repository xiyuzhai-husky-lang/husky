use crate::*;
use husky_dec_signature::helpers::projs::dec_var_full_projs;
use husky_devsoul::helpers::{
    DevsoulAnchor, DevsoulChart, DevsoulChartDim0, DevsoulChartDim1, DevsoulJointPedestal,
    DevsoulOrderedVarMap, DevsoulPedestal, DevsoulStaticVarMap,
};
use husky_ki_repr::repr::KiDomainRepr;
use husky_linket_impl::pedestal::JointPedestal;
use husky_trace_protocol::chart::{ChartDim0, ChartDim1};
use husky_trace_protocol::{anchor::Anchor, chart::Chart};
use smallvec::SmallVec;
use vec_like::SmallVecSet;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn with_var_anchors<R>(
        &self,
        var_anchors: impl IntoIterator<Item = (ItemPath, DevsoulAnchor<Devsoul>)>,
        f: impl FnMut(&DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChart<Devsoul, R>> {
        let db = self.db();
        let mut locked: SmallVecSet<ItemPathIdInterface, 2> = Default::default();
        let var_anchors: SmallVec<
            [(
                ItemPath,
                DevsoulAnchor<Devsoul>,
                SmallVecSet<ItemPathIdInterface, 2>,
            ); 2],
        > = var_anchors
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
        let number_of_generics = var_anchors
            .iter()
            .filter(|&(path, anchor, ref locked)| anchor.is_generic())
            .count();
        match number_of_generics {
            0 => self
                .with_var_anchors_aux0(Default::default(), &var_anchors, f)
                .map(Into::into),
            1 => self
                .with_var_anchors_aux1(Default::default(), &var_anchors, f)
                .map(Into::into),
            2 => {
                todo!()
            }
            _ => todo!(),
        }
    }

    pub fn with_var_anchors_aux0<R>(
        &self,
        mut var_map: DevsoulOrderedVarMap<Devsoul>,
        remaining_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut(&DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChartDim0<Devsoul, R>> {
        let db = self.db();
        for &(path, anchor, ref locked) in remaining_vars {
            let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path else {
                todo!()
            };
            let linket_impl = self
                .comptime
                .linket_impl(Linket::new_var(major_form_path, db));
            let Anchor::Specific(var_id) = anchor else {
                unreachable!()
            };
            todo!()
        }
        let joint_pedestal = JointPedestal::new(var_map);
        let r = f(&joint_pedestal)?;
        Some((joint_pedestal, r))
    }

    pub fn with_var_anchors_aux1<R>(
        &self,
        mut var_map: DevsoulOrderedVarMap<Devsoul>,
        remaining_vars: &[(
            ItemPath,
            DevsoulAnchor<Devsoul>,
            SmallVecSet<ItemPathIdInterface, 2>,
        )],
        mut f: impl FnMut(&DevsoulJointPedestal<Devsoul>) -> Option<R>,
    ) -> Option<DevsoulChartDim1<Devsoul, R>> {
        let &[(path, anchor, ref locked), ref remaining_vars @ ..] = remaining_vars else {
            unreachable!()
        };
        let db = self.db();
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path else {
            todo!()
        };
        let linket_impl = self
            .comptime
            .linket_impl(Linket::new_var(major_form_path, db));
        match anchor {
            Anchor::Specific(var_id) => {
                var_map.insert(((*path).into(), var_id));
                linket_impl
                    .with_var_id(var_id, locked, || {
                        self.with_var_anchors_aux1(var_map, remaining_vars, f)
                    })
                    .ok()
                    .flatten()
            }
            Anchor::Generic { page_limit } => {
                let iter = linket_impl.all_var_ids(locked).filter_map(|var_id| {
                    let mut var_map = var_map.clone();
                    var_map.insert(((*path).into(), var_id));
                    linket_impl
                        .with_var_id(var_id, locked, || {
                            self.with_var_anchors_aux0(var_map, remaining_vars, &mut f)
                        })
                        .ok()
                        .flatten()
                });
                Some(match page_limit {
                    Some(page_limit) => iter.take(page_limit).collect(),
                    None => iter.collect(),
                })
            }
        }
    }
}
