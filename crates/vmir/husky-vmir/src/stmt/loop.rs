use super::{expr::VmirExprIdx, *};
use husky_entity_path::path::major_item::ty::PreludeIntTypePath;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_hir_eager_expr::{
    variable::runtime::HirEagerRuntimeVariableIdx, HirEagerForBetweenLoopBoundary,
    HirEagerForBetweenParticulars, HirEagerForBetweenRange,
};
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_place::place::idx::PlaceIdx;
use husky_value::vm_control_flow::VmControlFlow;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct VmirForBetweenParticulars<LinketImpl: IsLinketImpl> {
    for_loop_variable_ty_path: PreludeIntTypePath,
    range: VmirForBetweenRange<LinketImpl>,
}

impl<LinketImpl: IsLinketImpl> VmirForBetweenParticulars<LinketImpl> {
    pub fn range(&self) -> &VmirForBetweenRange<LinketImpl> {
        &self.range
    }

    pub fn for_loop_variable_ty_path(&self) -> PreludeIntTypePath {
        self.for_loop_variable_ty_path
    }
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenParticulars {
    type Output = VmirForBetweenParticulars<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenParticulars {
            for_loop_variable_ty_path: self.for_loop_variable_ty_path,
            range: self.range.to_vmir(builder),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db]
pub struct VmirForBetweenRange<LinketImpl: IsLinketImpl> {
    pub initial_boundary: VmirForBetweenLoopBoundary<LinketImpl>,
    pub final_boundary: VmirForBetweenLoopBoundary<LinketImpl>,
    pub step: LoopStep,
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenRange {
    type Output = VmirForBetweenRange<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenRange {
            initial_boundary: self.initial_boundary.to_vmir(builder),
            final_boundary: self.final_boundary.to_vmir(builder),
            step: self.step,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VmirForBetweenLoopBoundary<LinketImpl: IsLinketImpl> {
    pub bound_expr: Option<VmirExprIdx<LinketImpl>>,
    pub kind: LoopBoundaryKind,
}

impl<LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &HirEagerForBetweenLoopBoundary {
    type Output = VmirForBetweenLoopBoundary<LinketImpl>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: husky_linktime::IsLinktime<LinketImpl = LinketImpl>,
    {
        VmirForBetweenLoopBoundary {
            bound_expr: self.bound_expr.as_ref().map(|expr| expr.to_vmir(builder)),
            kind: self.kind,
        }
    }
}

impl<LinketImpl: IsLinketImpl> VmirStmtIdx<LinketImpl> {
    pub(super) fn eval_for_between<'comptime>(
        self,
        stmts: VmirStmtIdxRange<LinketImpl>,
        particulars: &VmirForBetweenParticulars<LinketImpl>,
        for_loop_variable_idx: HirEagerRuntimeVariableIdx,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<LinketImpl> {
        use VmControlFlow::*;

        let range = particulars.range();
        let initial_bound = range
            .initial_boundary
            .bound_expr
            .map(|expr| expr.eval(None, ctx));
        let final_bound = particulars
            .range()
            .final_boundary
            .bound_expr
            .map(|expr| expr.eval(None, ctx));
        let step: i64 = match particulars.range().step {
            LoopStep::Constant(step) => step as i64,
        };
        let mut for_loop_variable = match initial_bound {
            Some(initial_bound) => {
                let initial_bound = initial_bound?.to_i64();
                match range.initial_boundary.kind {
                    LoopBoundaryKind::UpperOpen | LoopBoundaryKind::LowerOpen => {
                        initial_bound + step
                    }
                    LoopBoundaryKind::UpperClosed | LoopBoundaryKind::LowerClosed => initial_bound,
                }
            }
            None => 0,
        };
        if step > 0 {
            let limit: i64 = match final_bound {
                Some(final_bound) => {
                    final_bound?.to_i64()
                        + match range.final_boundary.kind {
                            LoopBoundaryKind::UpperOpen => 0,
                            LoopBoundaryKind::UpperClosed => -1,
                            LoopBoundaryKind::LowerOpen | LoopBoundaryKind::LowerClosed => {
                                unreachable!()
                            }
                        }
                }
                None => 0,
            };
            let mut loop_index: usize = 0;
            while for_loop_variable < limit {
                let for_loop_variable_value = convert_for_loop_variable::<LinketImpl>(
                    for_loop_variable,
                    particulars.for_loop_variable_ty_path(),
                );
                ctx.set_variable(for_loop_variable_idx, for_loop_variable_value);
                match ctx.eval_loop_inner(self, stmts, loop_index, |ctx| {
                    stmts.eval(ctx).map(|c| c.into())
                }) {
                    Continue(c) => (),
                    LoopContinue => (),
                    LoopExit(e) => {
                        let () = e.into();
                        break;
                    }
                    Return(r) => return Return(r),
                    Throw(e) => return Throw(e),
                };
                for_loop_variable += step;
                loop_index += 1;
            }
        } else if step < 0 {
            todo!()
        } else {
            panic!()
        }
        Continue(().into())
    }
}

fn convert_for_loop_variable<LinketImpl: IsLinketImpl>(
    for_loop_variable: i64,
    ty_path: PreludeIntTypePath,
) -> LinketImplThawedValue<LinketImpl> {
    match ty_path {
        PreludeIntTypePath::I8 => i8::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::I16 => i16::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::I32 => i32::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::I64 => i64::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::I128 => i128::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::ISize => isize::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::U8 => u8::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::U16 => u16::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::U32 => u32::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::U64 => u64::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::U128 => u128::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::USize => usize::try_from(for_loop_variable).unwrap().into(),
        PreludeIntTypePath::R8 => LinketImplThawedValue::<LinketImpl>::from_r8(
            u8::try_from(for_loop_variable).unwrap().into(),
        ),
        PreludeIntTypePath::R16 => LinketImplThawedValue::<LinketImpl>::from_r16(
            u16::try_from(for_loop_variable).unwrap().into(),
        ),
        PreludeIntTypePath::R32 => LinketImplThawedValue::<LinketImpl>::from_r32(
            u32::try_from(for_loop_variable).unwrap().into(),
        ),
        PreludeIntTypePath::R64 => LinketImplThawedValue::<LinketImpl>::from_r64(
            u64::try_from(for_loop_variable).unwrap().into(),
        ),
        PreludeIntTypePath::R128 => LinketImplThawedValue::<LinketImpl>::from_r128(
            u128::try_from(for_loop_variable).unwrap().into(),
        ),
        PreludeIntTypePath::RSize => LinketImplThawedValue::<LinketImpl>::from_rsize(
            u64::try_from(for_loop_variable).unwrap().into(),
        ),
    }
}
