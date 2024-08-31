use husky_value_interface::IsValue;

use crate::*;

pub trait IsStaticVar<VarId>: 'static
where
    VarId: std::fmt::Debug + Copy + Eq + 'static,
{
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn ids<'a>(locked: &'a [ItemPathIdInterface]) -> Box<dyn Iterator<Item = VarId> + 'a> {
        if locked.contains(&Self::item_path_id_interface()) {
            return Box::new([Self::get_id()].into_iter());
        }
        Box::new(unsafe { Self::ids_aux(locked) })
    }

    unsafe fn ids_aux(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = VarId>;
    fn get_id() -> VarId;

    /// returns a restore if okay,
    ///
    /// otherwise, it's guaranteed that nothing changes on the binary side if err
    unsafe fn try_replace_id_aux(
        new: VarId,
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<VarId, impl FnOnce() + 'static>;

    unsafe fn try_replace_id(
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
        Self::try_replace_id_aux(new, locked)
    }

    fn with_id<R>(
        id: VarId,
        locked: &[ItemPathIdInterface],
        f: impl FnOnce() -> R,
    ) -> StaticVarResult<VarId, R> {
        let restore = unsafe { Self::try_replace_id(id, locked) }?;
        let r = f();
        restore();
        Ok(r)
    }

    type Value: IsValue;

    fn get_value() -> Self::Value;
}

/// this is a mild error, sometimes intentionally triggered
#[derive(Debug)]
pub enum StaticVarError<VarId> {
    ReplaceLocked {
        locked_path: ItemPathIdInterface,
        old: VarId,
        new: VarId,
    },
}

pub type StaticVarResult<VarId, T> = Result<T, StaticVarError<VarId>>;
