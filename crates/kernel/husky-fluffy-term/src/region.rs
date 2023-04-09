mod expectations;
mod terms;

pub(crate) use self::expectations::*;
pub(crate) use self::terms::*;

use crate::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FluffyTermRegion {
    terms: FluffyTerms,
    expectations: (),
}

impl FluffyTermRegion {
    pub fn terms(&self) -> &FluffyTerms {
        &self.terms
    }

    pub fn finalize_unresolved_term_table(&mut self, db: &dyn FluffyTermDb) {
        todo!()
    }
}
