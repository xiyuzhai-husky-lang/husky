use crate::pedestal::StandardPedestal;
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_linkage_impl::standard::StandardLinkageImpl;

pub struct StandardDevsoulInterface {}

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinkageImpl = StandardLinkageImpl<StandardPedestal>;

    fn eval_context() -> husky_devsoul_interface::DevEvalContext<Self::LinkageImpl> {
        todo!()
    }
}
