use husky_linket_impl::{
    eval_context::IsDevRuntimeInterface, linket_impl::IsLinketImpl, LinketImplVmControlFlowThawed,
};
use husky_linktime::IsLinktime;

// ad hoc place, where to move?
pub trait IsVmRuntime<LinketImpl: IsLinketImpl>: IsDevRuntimeInterface<LinketImpl> {
    type Linktime: IsLinktime<LinketImpl = LinketImpl>;
    fn linktime(&self) -> &Self::Linktime;
    // assuming pedestal is already set properly
    fn eval_val(
        &self,
        major_form_path: husky_entity_path::path::major_item::form::MajorFormPath,
    ) -> LinketImplVmControlFlowThawed<LinketImpl>;
}
