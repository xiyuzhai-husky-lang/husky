/// We use `trait` instead of `class` to avoid confusion with the mathematical concept `class`.
// TODO: ad hoc implementation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VdTraitItemPath {
    GroupMul,
    AbelianGroupAdd,
    RingAdd,
    RingMul,
    RingPower,
}

impl VdTraitItemPath {
    pub const GROUP_MUL: Self = VdTraitItemPath::GroupMul;
    pub const ABELIAN_GROUP_ADD: Self = VdTraitItemPath::AbelianGroupAdd;
    pub const RING_ADD: Self = VdTraitItemPath::RingAdd;
    pub const RING_MUL: Self = VdTraitItemPath::RingMul;
    pub const RING_POWER: Self = VdTraitItemPath::RingPower;
}
