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
    ki_repr::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface},
    HuskyIngredientIndex, HuskyJarIndex, IsDevRuntime, IsLinkageImpl, LinkageImplKiControlFlow,
};
use husky_entity_path::path::{major_item::MajorItemPath, ItemPath};
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_linkage::linkage::Linkage;
use husky_vfs::{error::VfsResult, path::linktime_target_path::LinktimeTargetPath};
use std::{convert::Infallible, path::Path};

/// Dropping libraries or linkage_impls before runtime storage will lead to segmentation fault
///
/// so it's necessary to pub `storage` field before `comptime`
pub struct DevRuntime<Devsoul: IsDevsoul> {
    config: DevRuntimeConfig<Devsoul>,
    storage: Devsoul::RuntimeStorage,
    comptime: DevComptime<Devsoul>,
}

impl<Devsoul: IsDevsoul> DevRuntime<Devsoul> {
    pub fn new(
        target_crate: impl AsRef<Path>,
        config: Option<DevRuntimeConfig<Devsoul>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate)?,
        })
    }

    pub fn db(&self) -> &::salsa::Db {
        self.comptime.db()
    }

    pub fn comptime_target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.comptime.linktime_target_path()
    }

    fn get_or_try_init_ki_value(
        &self,
        ki: husky_ki::Ki,
        static_var_deps: &husky_ki_repr::static_var_deps::KiStaticVarDeps,
        f: impl FnOnce() -> LinkageImplKiControlFlow<Devsoul::LinkageImpl>,
    ) -> LinkageImplKiControlFlow<Devsoul::LinkageImpl> {
        self.storage.get_or_try_init_ki_value(
            ki,
            static_var_deps
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
        let linkage = Linkage::new_static_var(path, db);
        let linkage_impl = self.comptime.linkage_impl(linkage);
        linkage_impl.get_static_var_id()
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

impl<Devsoul: IsDevsoul> IsDevRuntime<Devsoul::LinkageImpl> for DevRuntime<Devsoul> {
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_ingredient_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        f: impl FnOnce() -> DevsoulValueResult<Devsoul>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        let (ki, static_var_deps) = self
            .comptime
            .ingredient_ki_and_static_var_deps(jar_index, ingredient_index);
        self.get_or_try_init_ki_value(ki, static_var_deps, f)
    }

    fn eval_ingredient(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
    ) -> DevsoulKiControlFlow<Devsoul> {
        self.eval_ki_repr(
            self.comptime
                .ingredient_ki_repr(jar_index, ingredient_index),
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
        self.get_or_try_init_ki_value(ki_repr.ki(db), ki_repr.static_var_deps(db), || {
            f(ki_domain_repr)
        })
    }

    fn eval_memo_field(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> DevsoulKiControlFlow<Devsoul>,
    ) -> DevsoulKiControlFlow<Devsoul> {
        self.storage
            .get_or_try_init_memo_field_value(jar_index, ingredient_index, slf, f)
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
                    .linkage_impl(Linkage::new_enum_index_presenter(
                        path.parent_ty_path(db),
                        db,
                    ))
                    .enum_index_value_presenter();
                DevsoulValue::<Devsoul>::from_enum_index(path.index(db).raw(), presenter)
            }
        }
    }
}
