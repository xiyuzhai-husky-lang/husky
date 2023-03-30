mod adjective;
mod adverb;
mod article;
mod conjunction;
mod noun;
mod preposition;
mod pronoun;
mod verb;

pub use self::adjective::*;
pub use self::adverb::*;
pub use self::article::*;
pub use self::conjunction::*;
pub use self::noun::*;
pub use self::preposition::*;
pub use self::pronoun::*;
pub use self::verb::*;

/// parts of speech in English
/// this identifies the precise meaning of the pos
#[derive(Debug, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum Pos {
    Noun(Noun),
    Pronoun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Article,
}

pub trait IntoPos: Into<Pos> {}

pub struct IndexedPos<P>
where
    P: IntoPos,
{
    pos: P,
    token_idx: PosIndex,
}

pub struct PosIndex(u32);
