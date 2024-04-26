use crate::SemExprBuilder;
use husky_entity_path::TypeVariantPath;
use husky_fly_term::FlyTerm;
use husky_syn_expr::{SynPatternData, SynPatternIdx};

struct PatternProduct {
    patterns: Vec<SynPatternIdx>,
}

struct PatternProductChoices {
    products: Vec<PatternProduct>,
}

#[derive(Debug, Clone, Copy)]
pub enum Refutation {
    All { ty: FlyTerm },
    TypeVariant { ty: FlyTerm, path: TypeVariantPath },
}

pub struct RefutationProduct {
    refutations: Vec<Refutation>,
}

pub enum SemPatternRefutabilityError {
    Original(OriginalSemPatternRefutabilityError),
    Derived(DerivedSemPatternRefutabilityError),
}

pub enum OriginalSemPatternRefutabilityError {}

pub enum DerivedSemPatternRefutabilityError {
    Todo,
}

pub type SemPatternRefutabilityResult<T> = Result<T, ()>;

impl<'a> SemExprBuilder<'a> {
    /// Firstly, try to see if one choice covers everything;
    /// secondly, distribute the problem to each factor.
    ///
    /// If the following matches agains target `(A, B)`
    /// ```husky
    /// | (patt11, patt12)
    /// | (patt21, patt22)
    /// ```
    ///,
    /// we will first see if either `(patt11, patt12)` or `(patt21, patt22)` will cover everything,
    /// then we will reduce the problem to two subproblems
    /// - refuting `patt11 | patt21` against `A`
    /// - refuting `patt12 | patt22` against `B`
    ///
    /// but this is wrong haha,
    ///
    /// it remains to see what's the best way to do it
    fn refute_product(
        &self,
        pattern_product_choices: PatternProductChoices,
        refutation_product: RefutationProduct,
    ) -> SemPatternRefutabilityResult<bool> {
        for product in &pattern_product_choices.products {
            if self.is_product_trivial(product)? {
                return Ok(true);
            }
        }
        let n = refutation_product.refutations.len();
        for i in 0..n {
            let pattern_choices = pattern_product_choices
                .products
                .iter()
                .map(|product| product.patterns[i])
                .collect();
            if !self.refute(pattern_choices, refutation_product.refutations[i])? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn is_product_trivial(&self, product: &PatternProduct) -> SemPatternRefutabilityResult<bool> {
        for pattern in &product.patterns {
            match self.syn_expr_region_data().pattern_expr_arena()[pattern] {
                SynPatternData::Ident { .. } => (),
                _ => return Ok(false),
            }
        }
        Ok(true)
    }

    fn refute(
        &self,
        choices: Vec<SynPatternIdx>,
        refutation: Refutation,
    ) -> SemPatternRefutabilityResult<bool> {
        // ```
        // | Animal::Cat { weight, height }
        // | Animal::Dog { weight, height }
        // ```
        match refutation {
            Refutation::All { ty } => {
                todo!("look into what kind of type `ty` is, and then decompose it and use refute_product")
            }
            Refutation::TypeVariant { ty, path } => {
                todo!(
                    "look into what kind of type variant `path` of `ty` is, and then decompose it and use refute_product"
                )
            }
        }
    }
}
