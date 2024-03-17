use husky_task_interface::IsLinkageImpl;
use idx_arena::ArenaIdx;

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Todo(LinkageImpl),
}

pub type VmirRestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirRestructivePatternData<LinkageImpl>>;
