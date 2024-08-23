use crate::*;

pub trait IsStaticVar<StaticVarId>
where
    StaticVarId: std::fmt::Debug + Copy + Eq,
{
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn ids(locked: &[ItemPathIdInterface]) -> impl Iterator<Item = StaticVarId>;
    fn get_id() -> StaticVarId;
    /// use with_id instead
    unsafe fn replace_id(id: StaticVarId) -> Option<StaticVarId>;

    fn with_id<R>(id: StaticVarId, locked: &[ItemPathIdInterface], f: impl FnOnce() -> R) -> R {
        if locked.contains(&Self::item_path_id_interface()) {
            if Self::get_id() != id {
                unreachable!("Serious error. Shouldn't change locked static vars")
            }
            return f();
        }
        unsafe {
            let old = Self::replace_id(id);
            let r = f();
            if let Some(old) = old {
                let new = Self::replace_id(old);
                assert_eq!(new, Some(id));
            }
            r
        }
    }
}
