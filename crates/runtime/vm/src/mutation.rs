use word::Identifier;

use crate::*;

#[derive(Debug, Clone)]
pub struct MutationData<'eval> {
    pub varname: Identifier,
    pub before: StackValueSnapshot<'eval>,
    pub after: StackValueSnapshot<'eval>,
}
