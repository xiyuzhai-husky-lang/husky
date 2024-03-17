use super::*;
use husky_task_interface::IsLinkageImpl;
use idx_arena::{Arena, ArenaIdx};

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Some,
    Todo(LinkageImpl),
    Unit,
}

pub type VmirRestructivePatternArena<LinkageImpl> = Arena<VmirRestructivePatternData<LinkageImpl>>;
pub type VmirRestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirRestructivePatternData<LinkageImpl>>;

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_restructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirRestructivePatternIdx<Linktime::LinkageImpl> {
        let pattern = match *self.hir_eager_pattern_arena()[hir_eager_pattern].data() {
            HirEagerPatternData::Literal(_) => todo!(),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => todo!(),
            HirEagerPatternData::Unit(_) => VmirRestructivePatternData::Unit,
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => todo!(),
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => VmirRestructivePatternData::Some,
        };
        self.alloc_restructive_pattern(pattern)
    }
}
