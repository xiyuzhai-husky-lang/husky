use super::*;
use either::*;
use husky_control_flow_utils::require;
use husky_place::place::idx::PlaceIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

/// takes ownership of the match src, destruct it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirDestructivePatternData<LinketImpl: IsLinketImpl> {
    Some,
    Todo(LinketImpl),
}

pub type VmirDestructivePatternArena<LinketImpl> = Arena<VmirDestructivePatternData<LinketImpl>>;
pub type VmirDestructivePatternIdx<LinketImpl> = ArenaIdx<VmirDestructivePatternData<LinketImpl>>;
pub type VmirDestructivePatternIdxRange<LinketImpl> =
    ArenaIdxRange<VmirDestructivePatternData<LinketImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VmirDestructivePattern<LinketImpl: IsLinketImpl> {
    Default(Option<PlaceIdx>) = 1,
    Or(VmirDestructivePatternIdxRange<LinketImpl>),
    Other(VmirDestructivePatternIdx<LinketImpl>),
}

#[test]
fn vmir_destructive_pattern_size_works() {
    use husky_virtual_linket_impl::VirtualLinketImpl;

    assert_eq!(
        std::mem::size_of::<VmirDestructivePattern<VirtualLinketImpl>>(),
        std::mem::size_of::<Option<VmirDestructivePattern<VirtualLinketImpl>>>(),
    )
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_destructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> Option<VmirDestructivePattern<Linktime::LinketImpl>> {
        require!(hir_eager_pattern
            .entry(self.hir_eager_pattern_arena())
            .is_destructive());
        let pattern = self.build_destructive_pattern_aux(hir_eager_pattern);
        Some(match pattern {
            Left(pattern) => pattern,
            Right(pattern_data) => {
                VmirDestructivePattern::Other(self.alloc_destructive_pattern(pattern_data))
            }
        })
    }

    pub(super) fn build_destructive_pattern_aux(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> Either<
        VmirDestructivePattern<Linktime::LinketImpl>,
        VmirDestructivePatternData<Linktime::LinketImpl>,
    > {
        match *self.hir_eager_pattern_arena()[hir_eager_pattern].data() {
            HirEagerPatternData::Literal(_) => todo!(),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
            } => Left(VmirDestructivePattern::Default(None /* ad hoc */)),
            HirEagerPatternData::UnitPath(_) => todo!(),
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => todo!(),
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => Right(VmirDestructivePatternData::Some),
        }
    }
}

impl<LinketImpl: IsLinketImpl> VmirDestructivePattern<LinketImpl> {
    pub(crate) fn take_value<'comptime>(
        self,
        value: LinketImplThawedValue<LinketImpl>,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) {
        match self {
            VmirDestructivePattern::Default(place) => match place {
                Some(place) => ctx.init_place(place, value),
                None => (),
            },
            VmirDestructivePattern::Or(_) => todo!(),
            VmirDestructivePattern::Other(_) => todo!(),
        }
    }
}
