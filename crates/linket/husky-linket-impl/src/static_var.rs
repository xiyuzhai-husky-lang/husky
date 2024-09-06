use husky_value::IsValue;
use serde::{Deserialize, Serialize};

use crate::*;

pub trait IsStaticVar<VarId>: 'static
where
    VarId: std::fmt::Debug + Copy + Eq + 'static,
{
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn page_var_ids<'a>(
        locked: &'a [ItemPathIdInterface],
        page_start: VarId,
        page_limit: Option<usize>,
    ) -> Box<dyn Iterator<Item = VarId> + 'a> {
        if locked.contains(&Self::item_path_id_interface()) {
            return Box::new([Self::get_id()].into_iter());
        }
        Box::new(unsafe { Self::page_var_ids_aux(locked) })
    }

    fn page_var_ids_aux(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = VarId>;

    fn default_page_start(locked: &[ItemPathIdInterface]) -> StaticVarResult<VarId, VarId>;

    fn get_id() -> VarId;

    fn try_set_var_id(
        new: VarId,
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<VarId, impl FnOnce() + 'static> {
        let item_path_id_interface = Self::item_path_id_interface();
        if locked.contains(&item_path_id_interface) {
            let old = Self::get_id();
            if old != new {
                Err(StaticVarError::ReplaceLocked {
                    locked_path: item_path_id_interface,
                    old,
                    new,
                })?
            }
        }
        Self::try_set_var_id_aux(new, locked)
    }

    /// returns a restore if okay,
    ///
    /// otherwise, it's guaranteed that nothing changes on the binary side if err
    fn try_set_var_id_aux(
        new: VarId,
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<VarId, impl FnOnce() + 'static>;

    fn try_set_default_var_id(
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<VarId, impl FnOnce() + 'static>;

    fn with_var_id<R>(
        var_id: VarId,
        locked: &[ItemPathIdInterface],
        f: impl FnOnce() -> R,
    ) -> StaticVarResult<VarId, R> {
        let restore = unsafe { Self::try_set_var_id(var_id, locked) }?;
        let r = f();
        restore();
        Ok(r)
    }

    type Value: IsValue;

    fn get_value() -> Self::Value;
}

/// this is a mild error, sometimes intentionally triggered
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StaticVarError<VarId> {
    ReplaceLocked {
        locked_path: ItemPathIdInterface,
        old: VarId,
        new: VarId,
    },
}

pub type StaticVarResult<VarId, T> = Result<T, StaticVarError<VarId>>;
