#![feature(negative_impls)]
#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_devsoul::{
    devsoul::IsDevsoul,
    helpers::{DevsoulException, DevsoulStaticVarId, DevsoulValue},
};
use husky_devsoul::{
    devsoul::IsRuntimeStorage,
    helpers::{DevsoulKiControlFlow, DevsoulValueResult},
};
use husky_devsoul_interface::{
    item_path::ItemPathIdInterface,
    ki_repr::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface},
    DevEvalContext, IsDevRuntime, IsLinketImpl, LinketImplKiControlFlow,
};
use husky_entity_path::path::{major_item::MajorItemPath, ItemPath, ItemPathId};
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_linket::linket::Linket;
use husky_vfs::{error::VfsResult, path::linktime_target_path::LinktimeTargetPath};
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

    pub(crate) fn eval_context(&self) -> DevEvalContext<Devsoul::LinketImpl> {
        DevEvalContext::new(unsafe { husky_wild_utils::arb_ref(self) })
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.comptime.linktime_target_path()
    }

    fn get_or_try_init_ki_value(
        &self,
        ki: husky_ki::Ki,
        var_deps: &husky_ki_repr::var_deps::KiStaticVarDeps,
        f: impl FnOnce() -> LinketImplKiControlFlow<Devsoul::LinketImpl>,
    ) -> LinketImplKiControlFlow<Devsoul::LinketImpl> {
        self.storage.get_or_try_init_ki_value(
            ki,
            var_deps
                .iter()
                .map(|&path| (path, self.get_static_var_id(path))),
            f,
            self.db(),
        )
    }

    fn get_static_var_id(&self, path: ItemPath) -> DevsoulStaticVarId<Devsoul> {
        let db = self.db();
        let ItemPath::MajorItem(MajorItemPath::Form(path)) = path else {
            todo!()
        };
        let linket = Linket::new_static_var(path, db);
        let linket_impl = self.comptime.linket_impl(linket);
        linket_impl.get_static_var_id()
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

impl<Devsoul: IsDevsoul> IsDevRuntime<Devsoul::LinketImpl> for DevRuntime<Devsoul> {
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_eager_val_with(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: <Devsoul::LinketImpl as IsLinketImpl>::Pedestal,
        f: fn() -> LinketImplKiControlFlow<Devsoul::LinketImpl>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        let db = self.db();
        let val_item_path_id: ItemPathId = val_item_path_id_interface.into();
        let val_item_path = val_item_path_id.item_path(db);
        todo!()
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
    ) -> husky_devsoul_interface::ki_control_flow::KiControlFlow<
        (),
        Infallible,
        DevsoulException<Devsoul>,
    > {
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
        self.get_or_try_init_ki_value(ki_repr.ki(db), ki_repr.var_deps(db), || f(ki_domain_repr))
    }

    fn eval_memo_field(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> DevsoulKiControlFlow<Devsoul>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        self.storage
            .get_or_try_init_memo_field_value(item_path_id_interface, slf, f)
    }

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> DevsoulValue<Devsoul> {
        use husky_value_interface::IsValue;

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
}
