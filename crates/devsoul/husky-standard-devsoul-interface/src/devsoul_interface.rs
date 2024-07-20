use crate::pedestal::StandardPedestal;
use husky_devsoul_interface::devsoul::IsDevsoulInterface;
use husky_linket_impl::standard::StandardLinketImpl;

pub struct StandardDevsoulInterface {}

impl IsDevsoulInterface for StandardDevsoulInterface {
    type LinketImpl = StandardLinketImpl<StandardPedestal>;

    fn eval_context() -> husky_devsoul_interface::DevEvalContext<Self::LinketImpl> {
        todo!()
    }
}
