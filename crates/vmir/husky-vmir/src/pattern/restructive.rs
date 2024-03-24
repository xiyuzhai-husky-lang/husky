use super::*;
use either::*;
use husky_place::place::idx::PlaceIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData<LinkageImpl: IsLinkageImpl> {
    Literal,
    Some,
    Todo(LinkageImpl),
    UnitPath,
}

pub type VmirRestructivePatternArena<LinkageImpl> = Arena<VmirRestructivePatternData<LinkageImpl>>;
pub type VmirRestructivePatternIdx<LinkageImpl> = ArenaIdx<VmirRestructivePatternData<LinkageImpl>>;
pub type VmirRestructivePatternIdxRange<LinkageImpl> =
    ArenaIdxRange<VmirRestructivePatternData<LinkageImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VmirRestructivePattern<LinkageImpl: IsLinkageImpl> {
    Default(Option<PlaceIdx>) = 1,
    Literal,
    UnitPath,
    OneOf(VmirRestructivePatternIdxRange<LinkageImpl>),
    Other(VmirRestructivePatternIdx<LinkageImpl>),
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_restructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirRestructivePattern<Linktime::LinkageImpl> {
        let pattern = self.build_restructive_pattern_aux(hir_eager_pattern);
        match pattern {
            Left(pattern) => pattern,
            Right(pattern) => {
                VmirRestructivePattern::Other(self.alloc_restructive_pattern(pattern))
            }
        }
    }

    pub(super) fn build_restructive_pattern_aux(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> Either<
        VmirRestructivePattern<Linktime::LinkageImpl>,
        VmirRestructivePatternData<Linktime::LinkageImpl>,
    > {
        match *self.hir_eager_pattern_arena()[hir_eager_pattern].data() {
            HirEagerPatternData::Literal(_) => Left(VmirRestructivePattern::Literal),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => Left(VmirRestructivePattern::Default(None /* ad hoc */)),
            HirEagerPatternData::UnitPath(path) => Left(VmirRestructivePattern::UnitPath),
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => {
                let options = options
                    .into_iter()
                    .map(|option| match self.build_restructive_pattern_aux(option) {
                        Left(pattern) => match pattern {
                            VmirRestructivePattern::Default(_) => todo!(),
                            VmirRestructivePattern::Literal => VmirRestructivePatternData::Literal,
                            VmirRestructivePattern::UnitPath => {
                                VmirRestructivePatternData::UnitPath
                            }
                            VmirRestructivePattern::OneOf(_) => todo!(),
                            VmirRestructivePattern::Other(_) => todo!(),
                        },
                        Right(pattern_data) => pattern_data,
                    })
                    .collect();
                let options = self.alloc_restructive_patterns(options);
                Left(VmirRestructivePattern::OneOf(options))
            }
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => Right(VmirRestructivePatternData::Some),
        }
    }
}
