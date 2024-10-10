use crate::*;
use idx_arena::Arena;

#[derive(Debug)]
pub enum MayuriSquadMember {
    Local,
    Remote,
}

pub type MayuriSquadMemberArena = Arena<MayuriSquadMember>;
