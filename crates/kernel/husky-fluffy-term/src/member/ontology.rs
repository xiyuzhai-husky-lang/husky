mod ethereal;
mod hollow;
mod solid;

use self::ethereal::*;
use self::hollow::*;
use self::solid::*;
use super::*;
use husky_word::Ident;

pub enum AssociatedItemDisambiguation {
    AssociatedFn,
}

impl FluffyTerm {
    pub fn disambiguate_associated_item(
        self,
        engine: &mut impl FluffyTermEngine,
        ident: Ident,
        all_available_traits: &[()],
    ) -> AssociatedItemDisambiguation {
        todo!()
    }
}
