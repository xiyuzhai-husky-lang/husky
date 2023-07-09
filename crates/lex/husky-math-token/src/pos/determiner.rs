#[enum_class::from_variants]
pub enum DeterminerToken {
    IndefiniteArticle(IndefiniteArticleToken),
    DefiniteArticle(DefiniteArticleToken),
    Demonstrative(DemonstrativeToken),
    Quantifier(QuantifierToken),
    DistributiveDeterminer(DistributiveDeterminerToken),
}

pub enum IndefiniteArticleToken {
    A,
    An,
    One,
}

pub enum DefiniteArticleToken {
    The,
}

pub enum DemonstrativeToken {
    This,
    That,
    These,
    Those,
}

pub enum QuantifierToken {
    All,
    Some,
    Many,
    Little,
    Few,
    No,
}

pub enum DistributiveDeterminerToken {
    Each,
    Every,
}
