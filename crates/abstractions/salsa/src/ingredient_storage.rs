use crate::ingredient::Ingredient;
use husky_wild_utils::{arb_mut, arb_ref};
use rustc_hash::FxHashMap;

#[derive(Default)]
pub struct IngredientStorage {
    map: FxHashMap<std::any::TypeId, Box<dyn Ingredient>>,
}

impl IngredientStorage {
    pub fn ingredient<I>(&self) -> &I
    where
        I: Ingredient,
    {
        unsafe {
            arb_ref(
                (&**self.map.get(&std::any::TypeId::of::<I>()).unwrap() as &dyn std::any::Any)
                    .downcast_ref()
                    .unwrap(),
            )
        }
    }

    pub fn ingredient_mut<I>(&mut self) -> &mut I
    where
        I: Ingredient,
    {
        unsafe {
            arb_mut(
                (&mut **self.map.get_mut(&std::any::TypeId::of::<I>()).unwrap()
                    as &mut dyn std::any::Any)
                    .downcast_mut()
                    .unwrap(),
            )
        }
    }
}
