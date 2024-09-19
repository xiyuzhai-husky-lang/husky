#![feature(result_flattening)]
#![feature(negative_impls)]
#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;
#[cfg(test)]
mod tests;
mod var;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_devsoul::{
    devsoul::IsDevsoul,
    helpers::{DevsoulTrackedException, DevsoulValue, DevsoulVarId},
};
use husky_devsoul::{
    devsoul::IsRuntimeStorage,
    helpers::{DevsoulKiControlFlow, DevsoulValueResult},
};
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{major_item::MajorItemPath, ItemPath, ItemPathId};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface};
use husky_linket::linket::Linket;
use husky_linket_impl::{
    eval_context::{DevEvalContext, IsDevRuntimeInterface},
    linket_impl::{IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedExceptedValue},
    pedestal::IsPedestal,
};
use husky_value::ki_control_flow::KiControlFlow;
use husky_vfs::{error::VfsResult, path::linktime_target_path::LinktimeTargetPath};
use husky_vm::eval::IsDevRuntime;
use husky_wild_utils::arb_ref;
use std::{
    convert::Infallible,
    path::Path,
    pin::{pin, Pin},
};

/// Dropping libraries or linket_impls before runtime storage will lead to segmentation fault
///
/// so it's necessary to pub `storage` field before `comptime`
pub struct DevRuntime<Devsoul: IsDevsoul> {
    config: DevRuntimeConfig<Devsoul>,
    storage: Devsoul::RuntimeStorage,
    comptime: DevComptime<Devsoul>,
}

impl<Devsoul> !Unpin for DevRuntime<Devsoul> {}

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn new(
        target_crate: impl AsRef<Path>,
        config: Option<DevRuntimeConfig<Devsoul>>,
    ) -> VfsResult<Pin<Box<Self>>> {
        let mut slf = Box::pin(Self {
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate)?,
        });
        slf.init();
        Ok(slf)
    }

    fn init(self: &Self) {
        self.comptime.init(unsafe { arb_ref(self) });
    }
}

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn db(&self) -> &::salsa::Db {
        self.comptime.db()
    }

    pub fn comptime_target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }

    pub(crate) fn dev_eval_context(&self) -> DevEvalContext<Devsoul::LinketImpl> {
        DevEvalContext::new(unsafe { husky_wild_utils::arb_ref(self) })
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.comptime.linktime_target_path()
    }

    fn get_or_try_init_ki_value(
        &self,
        ki_repr: KiRepr,
        f: impl FnOnce() -> LinketImplKiControlFlow<Devsoul::LinketImpl>,
    ) -> LinketImplKiControlFlow<Devsoul::LinketImpl> {
        let db = self.db();
        let ki = ki_repr.ki(db);
        self.storage
            .get_or_try_init_ki_value(ki, self.pedestal(ki_repr), f, self.db())
    }

    fn pedestal(&self, ki_repr: KiRepr) -> <Devsoul::LinketImpl as IsLinketImpl>::Pedestal {
        ki_repr
            .var_deps(self.db())
            .iter()
            .map(|&path| ((*path).into(), self.get_static_var_id(path)))
            .collect()
    }

    fn get_static_var_id(&self, path: ItemPath) -> DevsoulVarId<Devsoul> {
        let db = self.db();
        let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
            todo!()
        };
        let linket = Linket::new_var(path, db);
        let linket_impl = self.comptime.linket_impl(linket);
        linket_impl.static_var_id()
    }

    pub fn comptime(&self) -> &DevComptime<Devsoul> {
        &self.comptime
    }
}

impl<Devsoul: IsDevsoul> Default for DevRuntime<Devsoul>
where
    Devsoul::Linktime: Default,
{
    fn default() -> Self {
        Self {
            comptime: Default::default(),
            storage: Default::default(),
            config: Default::default(),
        }
    }
}

impl<Devsoul: IsDevsoul> IsDevRuntimeInterface<Devsoul::LinketImpl> for DevRuntime<Devsoul> {
    type ThawedSelf = Self;

    unsafe fn cast_to_thawed_self_static_ref(&self) -> &'static Self::ThawedSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_eager_val_with(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: <Devsoul::LinketImpl as IsLinketImpl>::Pedestal,
        f: fn() -> LinketImplKiControlFlow<Devsoul::LinketImpl>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        use ::husky_print_utils::p;
        use ::salsa::DebugWithDb;
        self.storage
            .get_or_try_init_val_value(val_item_path_id_interface, pedestal, f, self.db())
    }

    fn eval_lazy_val(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: <Devsoul::LinketImpl as IsLinketImpl>::Pedestal,
    ) -> DevsoulKiControlFlow<Devsoul> {
        let db = self.db();
        let val_item_path_id: ItemPathId = val_item_path_id_interface.into();
        let val_ki = match val_item_path_id.item_path(db) {
            ItemPath::Submodule(_, _) => todo!(),
            ItemPath::MajorItem(path) => match path {
                MajorItemPath::Type(_) => todo!(),
                MajorItemPath::Trait(_) => todo!(),
                MajorItemPath::Form(path) => KiRepr::new_val(path, db),
            },
            ItemPath::AssocItem(_) => todo!(),
            ItemPath::TypeVariant(_, _) => todo!(),
            ItemPath::ImplBlock(_) => todo!(),
            ItemPath::Attr(_, _) => todo!(),
            ItemPath::Script(_, _) => todo!(),
        };
        self.storage.get_or_try_init_val_value(
            val_item_path_id_interface,
            pedestal,
            || self.eval_ki_repr(val_ki),
            self.db(),
        )
    }

    fn eval_ki_repr_interface(
        &self,
        ki_repr_interface: KiReprInterface,
    ) -> DevsoulKiControlFlow<Devsoul> {
        self.eval_ki_repr(ki_repr_interface.into())
    }

    fn eval_ki_domain_repr_interface(
        &self,
        ki_domain_repr: KiDomainReprInterface,
    ) -> KiControlFlow<(), Infallible, DevsoulTrackedException<Devsoul>> {
        self.eval_ki_domain_repr(ki_domain_repr.into())
    }

    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        f: impl FnOnce(KiDomainReprInterface) -> DevsoulKiControlFlow<Devsoul>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        let db = self.db();
        let ki_repr: KiRepr = unsafe { std::mem::transmute(ki_repr) };
        let ki_domain_repr: KiDomainReprInterface =
            unsafe { std::mem::transmute(ki_repr.ki_domain_repr(db)) };
        self.get_or_try_init_ki_value(ki_repr, || f(ki_domain_repr))
    }

    fn eval_memo_field_with(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        __self: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> DevsoulKiControlFlow<Devsoul>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        self.storage
            .get_or_try_init_memo_field_value(item_path_id_interface, __self, f)
    }

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> DevsoulValue<Devsoul> {
        use husky_value::IsValue;

        let db = self.db();
        let val_runtime_constant: KiRuntimeConstant =
            unsafe { std::mem::transmute(val_runtime_constant) };
        match val_runtime_constant.data(db) {
            KiRuntimeConstantData::TypeVariantPath(path) => {
                let presenter = self
                    .comptime
                    .linket_impl(Linket::new_enum_index_presenter(
                        path.parent_ty_path(db),
                        db,
                    ))
                    .enum_index_value_presenter();
                DevsoulValue::<Devsoul>::from_enum_index(path.index(db).raw(), presenter)
            }
        }
    }

    /// there is room for optimization, see the definition of KiVarDeps
    fn eval_ki_pedestal(
        &self,
        ki_repr_interface: KiReprInterface,
    ) -> <Devsoul::LinketImpl as IsLinketImpl>::Pedestal {
        let db = self.db();
        let ki_repr: KiRepr = ki_repr_interface.into();
        ki_repr
            .var_deps(db)
            .iter()
            .map(|&var_dep_item_path| match var_dep_item_path {
                ItemPath::Submodule(_, _) => todo!(),
                ItemPath::MajorItem(var_dep_major_item_path) => match var_dep_major_item_path {
                    MajorItemPath::Type(_) => todo!(),
                    MajorItemPath::Trait(_) => todo!(),
                    MajorItemPath::Form(var_dep_major_form_path) => {
                        match var_dep_major_form_path.kind(db) {
                            MajorFormKind::Ritchie(_) => todo!(),
                            MajorFormKind::TypeAlias => todo!(),
                            MajorFormKind::TypeVar => todo!(),
                            MajorFormKind::Val => todo!(),
                            MajorFormKind::StaticMut => todo!(),
                            MajorFormKind::StaticVar => {
                                let linket = Linket::new_var(var_dep_major_form_path, db);
                                let linket_impl = self.comptime.linket_impl(linket);
                                let static_var_id = linket_impl.static_var_id();
                                ((*var_dep_item_path).into(), static_var_id)
                            }
                            MajorFormKind::Compterm => todo!(),
                            MajorFormKind::Conceptual => todo!(),
                        }
                    }
                },
                ItemPath::AssocItem(_) => todo!(),
                ItemPath::TypeVariant(_, _) => todo!(),
                ItemPath::ImplBlock(_) => todo!(),
                ItemPath::Attr(_, _) => todo!(),
                ItemPath::Script(_, _) => todo!(),
            })
            .collect()
    }

    fn eval_generic_gn_with<'a>(
        &'a self,
        ki_repr_interface: KiReprInterface,
        pedestal: <Devsoul::LinketImpl as IsLinketImpl>::Pedestal,
        f: Box<dyn FnOnce() -> LinketImplKiControlFlow<Devsoul::LinketImpl> + 'a>,
    ) -> LinketImplKiControlFlow<Devsoul::LinketImpl> {
        let db = self.db();
        let ki_repr: KiRepr = ki_repr_interface.into();
        let ki = ki_repr.ki(db);
        self.storage
            .get_or_try_init_generic_gn_value(ki, pedestal, f, db)
    }
}

impl<Devsoul: IsDevsoul> IsDevRuntime<Devsoul::LinketImpl> for DevRuntime<Devsoul> {
    type Linktime = Devsoul::Linktime;

    fn linktime(&self) -> &Self::Linktime {
        self.comptime().linktime()
    }
}
