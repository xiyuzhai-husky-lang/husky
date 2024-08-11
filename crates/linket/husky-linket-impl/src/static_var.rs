use crate::*;

pub trait IsStaticVar<StaticVarId>
where
    StaticVarId: std::fmt::Debug + Copy + Eq,
{
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn ids() -> impl Iterator<Item = StaticVarId>;
    fn get_id() -> StaticVarId;
    /// use with_id instead
    unsafe fn replace_id(id: StaticVarId) -> Option<StaticVarId>;

    fn with_id<R>(id: StaticVarId, f: impl FnOnce() -> R) -> R {
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
