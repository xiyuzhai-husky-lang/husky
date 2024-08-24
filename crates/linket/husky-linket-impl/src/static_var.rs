use crate::*;

pub trait IsStaticVar<StaticVarId>
where
    StaticVarId: std::fmt::Debug + Copy + Eq,
{
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn ids(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = StaticVarId>;
    fn get_id() -> StaticVarId;

    /// returns a restore if okay,
    ///
    /// otherwise, it's guaranteed that nothing changes on the binary side if err
    unsafe fn try_replace_id_aux(
        new: StaticVarId,
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<StaticVarId, impl FnOnce() + 'static>;

    unsafe fn try_replace_id(
        new: StaticVarId,
        locked: &[ItemPathIdInterface],
    ) -> StaticVarResult<StaticVarId, impl FnOnce() + 'static> {
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
        id: StaticVarId,
        locked: &[ItemPathIdInterface],
        f: impl FnOnce() -> R,
    ) -> StaticVarResult<StaticVarId, R> {
        let restore = unsafe { Self::try_replace_id(id, locked) }?;
        let r = f();
        restore();
        Ok(r)
    }
}

/// this is a mild error, sometimes intentionally triggered
#[derive(Debug)]
pub enum StaticVarError<StaticVarId> {
    ReplaceLocked {
        locked_path: ItemPathIdInterface,
        old: StaticVarId,
        new: StaticVarId,
    },
}

pub type StaticVarResult<StaticVarId, T> = Result<T, StaticVarError<StaticVarId>>;
