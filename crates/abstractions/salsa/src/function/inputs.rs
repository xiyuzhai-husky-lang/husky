use super::{Configuration, FunctionIngredient};
use crate::{runtime::local_state::QueryOrigin, AsIdWithDb};

impl<C> FunctionIngredient<C>
where
    C: Configuration,
{
    pub(super) fn origin(&self, key: C::Key) -> Option<QueryOrigin> {
        self.memo_map
            .get(key.as_id_with_db())
            .map(|m| m.revisions.origin.clone())
    }
}
