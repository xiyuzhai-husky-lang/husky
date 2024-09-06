use crate::*;
use husky_dec_signature::helpers::projs::dec_var_full_projs;
use husky_devsoul::helpers::{
    DevsoulAnchor, DevsoulChart, DevsoulChartDim0, DevsoulChartDim1, DevsoulJointPedestal,
    DevsoulOrderedVarMap, DevsoulPedestal, DevsoulStaticVarMap, DevsoulStaticVarResult,
};
use husky_ki_repr::repr::KiDomainRepr;
use husky_linket_impl::{pedestal::JointPedestal, static_var::StaticVarResult};
use husky_trace_protocol::chart::{ChartDim0, ChartDim1};
use husky_trace_protocol::{anchor::Anchor, chart::Chart};
use smallvec::SmallVec;
use vec_like::SmallVecSet;

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn with_var_id<R>(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        var_id: DevsoulVarId<Devsoul>,
        locked: &SmallVecSet<ItemPathIdInterface, 4>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        let mut locked1 = locked.clone();
        locked1.insert_new(item_path_id_interface).unwrap();
        let path_id: ItemPathId = item_path_id_interface.into();
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path_id.item_path(db)
        else {
            todo!()
        };
        let linket_impl = self
            .comptime
            .linket_impl(Linket::new_var(major_form_path, db));
        linket_impl.with_var_id(var_id, &locked, || f(locked1))
    }

    pub fn with_var_ids<R>(
        &self,
        var_ids: impl IntoIterator<Item = (ItemPathIdInterface, DevsoulVarId<Devsoul>)>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        self.with_var_ids_aux(var_ids.into_iter(), Default::default(), f)
    }

    fn with_var_ids_aux<R>(
        &self,
        mut var_ids: impl Iterator<Item = (ItemPathIdInterface, DevsoulVarId<Devsoul>)>,
        locked: SmallVecSet<ItemPathIdInterface, 4>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        match var_ids.next() {
            Some((item_path_id_interface, var_id)) => self
                .with_var_id(item_path_id_interface, var_id, &locked, |locked| {
                    self.with_var_ids_aux(var_ids, locked, f)
                })
                .flatten(),
            None => Ok(f(locked)),
        }
    }

    pub fn with_default_var_id<R>(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        locked: &SmallVecSet<ItemPathIdInterface, 4>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        let mut locked1 = locked.clone();
        locked1.insert_new(item_path_id_interface).unwrap();
        let path_id: ItemPathId = item_path_id_interface.into();
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path_id.item_path(db)
        else {
            todo!()
        };
        let linket_impl = self
            .comptime
            .linket_impl(Linket::new_var(major_form_path, db));
        linket_impl.with_default_var_id(&locked, || f(locked1))
    }

    pub fn with_default_var_ids<R>(
        &self,
        var_paths: impl IntoIterator<Item = ItemPathIdInterface>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        self.with_default_var_ids_aux(var_paths.into_iter(), Default::default(), f)
    }

    fn with_default_var_ids_aux<R>(
        &self,
        mut var_paths: impl Iterator<Item = ItemPathIdInterface>,
        locked: SmallVecSet<ItemPathIdInterface, 4>,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> StaticVarResult<DevsoulVarId<Devsoul>, R> {
        let db = self.db();
        match var_paths.next() {
            Some(var_path) => self
                .with_default_var_id(var_path, &locked, |locked| {
                    self.with_default_var_ids_aux(var_paths, locked, f)
                })
                .flatten(),
            None => Ok(f(locked)),
        }
    }

    // todo: change to result
    pub fn with_var_anchors<R>(
        &self,
        var_anchors: impl IntoIterator<Item = (ItemPathIdInterface, DevsoulAnchor<Devsoul>)>,
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
                |(item_path_id_interface, anchor)| -> Option<(
                    ItemPath,
                    DevsoulAnchor<Devsoul>,
                    SmallVecSet<ItemPathIdInterface, 2>,
                )> {
                    // todo: simplify using with_var_id
                    let item_path_id: ItemPathId = item_path_id_interface.into();
                    let ItemPath::MajorItem(MajorItemPath::Form(path)) = item_path_id.item_path(db)
                    else {
                        todo!()
                    };
                    match path.kind(db) {
                        MajorFormKind::TypeVar
                        | MajorFormKind::StaticMut
                        | MajorFormKind::Compterm
                        | MajorFormKind::Conceptual => todo!(),
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

    fn with_var_anchors_aux0<R>(
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

    fn with_var_anchors_aux1<R>(
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
            Anchor::Generic {
                page_start,
                page_limit,
            } => {
                let iter = linket_impl
                    .page_var_ids(locked, page_start, page_limit)
                    .filter_map(|var_id| {
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

    pub fn var_default_page_start_aux(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        locked: &SmallVecSet<ItemPathIdInterface, 4>,
    ) -> DevsoulStaticVarResult<Devsoul, DevsoulVarId<Devsoul>> {
        let db = self.db();
        let path_id: ItemPathId = item_path_id_interface.into();
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = path_id.item_path(db)
        else {
            todo!()
        };
        let linket_impl = self
            .comptime
            .linket_impl(Linket::new_var(major_form_path, db));
        linket_impl.var_default_page_start(locked)
    }

    pub fn with_pedestal<R>(
        &self,
        pedestal: &Devsoul::Pedestal,
        f: impl FnOnce(SmallVecSet<ItemPathIdInterface, 4>) -> R,
    ) -> DevsoulStaticVarResult<Devsoul, R> {
        self.with_var_ids(pedestal.var_ids(), f)
    }
}
