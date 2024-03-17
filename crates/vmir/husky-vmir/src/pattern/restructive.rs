use super::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Literal,
    Some,
    OneOf {
        options: VmirRestructivePatternIdxRange<LinkageImpl>,
    },
    Todo(LinkageImpl),
    Unit,
    Ident,
}

pub type VmirRestructivePatternArena<LinkageImpl> = Arena<VmirRestructivePatternData<LinkageImpl>>;
pub type VmirRestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirRestructivePatternData<LinkageImpl>>;
pub type VmirRestructivePatternIdxRange<LinkageImpl> =
    ArenaIdxRange<VmirRestructivePatternData<LinkageImpl>>;

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_restructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirRestructivePatternIdx<Linktime::LinkageImpl> {
        let pattern = self.build_restructive_pattern_aux(hir_eager_pattern);
        self.alloc_restructive_pattern(pattern)
    }

    pub(super) fn build_restructive_pattern_aux(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirRestructivePatternData<Linktime::LinkageImpl> {
        match *self.hir_eager_pattern_arena()[hir_eager_pattern].data() {
            HirEagerPatternData::Literal(_) => VmirRestructivePatternData::Literal,
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => VmirRestructivePatternData::Ident,
            HirEagerPatternData::Unit(_) => VmirRestructivePatternData::Unit,
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => {
                let options = options
                    .into_iter()
                    .map(|option| self.build_restructive_pattern_aux(option))
                    .collect();
                let options = self.alloc_restructive_patterns(options);
                VmirRestructivePatternData::OneOf { options }
            }
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => VmirRestructivePatternData::Some,
        }
    }
}
