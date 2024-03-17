use husky_task_interface::IsLinkageImpl;
use idx_arena::ArenaIdx;

/// takes ownership of the match src, destruct it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirDestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Todo(LinkageImpl),
}

pub type VmirDestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirDestructivePatternData<LinkageImpl>>;
