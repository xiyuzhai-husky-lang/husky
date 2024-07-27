use crate::*;

pub trait IsStaticVar<StaticVarId> {
    fn item_path_id_interface() -> ItemPathIdInterface;
    fn ids() -> impl Iterator<Item = StaticVarId>;
    fn get_id() -> StaticVarId;
    fn set_id(id: StaticVarId);
}
