use crate::vm::{Vm, VmMode};
use crate::*;
use history::VmHistory;
use husky_linktime::helpers::LinktimeThawedValue;
use husky_vmir::stmt::{VmirStmtIdx, VmirStmtIdxRange};

pub fn eval_linket_on_arguments<LinketImpl, Linktime>(
    linket: Linket,
    arguments: Vec<LinketImpl::Value>,
    mode: VmMode,
    db: &::salsa::Db,
    linktime: &Linktime,
    vmir_storage: &impl IsVmirStorage<LinketImpl>,
) -> Option<(
    LinketImplVmControlFlowThawed<LinketImpl>,
    VmHistory<LinketImpl>,
)>
where
    LinketImpl: IsLinketImpl,
    Linktime: IsLinktime<LinketImpl = LinketImpl>,
{
    let vmir_region = vmir_storage.linket_vmir_region(linket, db, linktime)?;
    let mut vm = vm::Vm::new_fresh(
        linket,
        arguments,
        mode,
        &vmir_region,
        db,
        linktime,
        vmir_storage,
    );
    let cf = vmir_region.root_expr().eval(None, &mut vm);
    let history = vm.to_history();
    Some((cf, history))
}

impl<'a, Linktime, VmirStorage> EvalVmir<'a, Linktime::LinketImpl> for Vm<'a, Linktime, VmirStorage>
where
    Linktime: IsLinktime,
    VmirStorage: IsVmirStorage<Linktime::LinketImpl>,
{
    fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn vmir_region(&self) -> &'a VmirRegion<Linktime::LinketImpl> {
        self.vmir_region
    }

    fn eval_expr(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => {
                // todo: do something?
                // set something to indicate the the expr starts to eval (children included)
                f(self)
            }
        }
    }

    fn eval_expr_itself(
        &mut self,
        expr: VmirExprIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => self.record_expr(expr, f),
        }
    }

    fn eval_stmts(
        &mut self,
        stmts: VmirStmtIdxRange<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => {
                // todo: do something?
                f(self)
            }
        }
    }

    fn eval_stmt(
        &mut self,
        stmt: VmirStmtIdx<Linktime::LinketImpl>,
        f: impl FnOnce(&mut Self) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl>,
    ) -> LinketImplVmControlFlowThawed<Linktime::LinketImpl> {
        match self.mode() {
            VmMode::Quick => f(self),
            VmMode::Record => self.record_stmt(stmt, f),
        }
    }

    fn access_place(
        &mut self,
        place_idx: PlaceIdx,
        qual: LinQual,
    ) -> LinktimeThawedValue<Linktime> {
        match qual {
            LinQual::Ref => todo!(),
            LinQual::RefMut => todo!(),
            LinQual::Transient => todo!(),
        }
    }

    fn init_place(&mut self, place_idx: PlaceIdx, value: LinktimeThawedValue<Linktime>) {
        self.place_thawed_values[place_idx.index()] = value
    }
}
