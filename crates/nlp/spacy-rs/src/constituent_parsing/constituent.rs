use pyo3::exceptions::PyValueError;

use super::*;
use crate::token::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Constituent {
    pub text: String,
    pub span: Span,
    pub children: Vec<Span>,
    pub labels: Vec<ConstituentLabel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstituentLabel {
    /// Sentence - represents a complete sentence
    /// Example: "The cat sat on the mat."
    Sentence,

    /// Noun Phrase - a phrase with a noun as its head
    /// Example: "the red car", "my friendly neighbor"
    NounPhrase,

    /// Verb Phrase - a phrase with a verb as its head
    /// Example: "is running fast", "quickly ate dinner"
    VerbPhrase,

    /// Prepositional Phrase - a phrase starting with a preposition
    /// Example: "in the house", "under the bridge"
    PrepositionalPhrase,

    /// Noun - a word that refers to a person, place, thing, or idea
    /// Example: "cat", "happiness", "London"
    Noun,

    /// Verb - a word that describes an action, state, or occurrence
    /// Example: "run", "sleep", "think"
    Verb,

    /// Adjective - a word that modifies or describes a noun
    /// Example: "happy", "tall", "blue"
    Adjective,

    /// Adverb - a word that modifies a verb, adjective, or other adverb
    /// Example: "quickly", "very", "well"
    Adverb,

    /// Determiner - a word that introduces a noun
    /// Example: "the", "a", "this", "some"
    Determiner,

    /// Preposition - a word that shows relationship between words
    /// Example: "in", "on", "at", "between"
    Preposition,

    /// Conjunction - a word that connects words, phrases, or clauses
    /// Example: "and", "but", "or"
    Conjunction,

    /// Subordinating Conjunction - introduces dependent clauses
    /// Example: "because", "although", "unless"
    SubordinatingConjunction,

    /// Complementizer - words that introduce subordinate clauses
    /// Example: "that" in "I think that she left"
    Complementizer,

    /// Interjection - exclamatory words
    /// Example: "oh!", "wow!", "ouch!"
    Interjection,

    /// Auxiliary Verb - helping verbs
    /// Example: "is", "have", "will", "must"
    AuxiliaryVerb,

    /// Relative Clause - dependent clause that modifies a noun
    /// Example: "who lives next door" in "The man who lives next door"
    RelativeClause,

    /// Infinitive Phrase - a phrase beginning with "to" + verb
    /// Example: "to run", "to eat quickly"
    InfinitivePhrase,

    /// Gerund Phrase - a phrase beginning with an -ing form of a verb
    /// Example: "running in the park", "eating ice cream"
    GerundPhrase,

    /// Participle Phrase - a phrase beginning with a participle
    /// Example: "broken by the fall", "singing loudly"
    ParticiplePhrase,

    /// Question - interrogative sentence
    /// Example: "Where are you going?", "What time is it?"
    Question,
}

impl TryFrom<&str> for ConstituentLabel {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "S" | "Sentence" => Ok(ConstituentLabel::Sentence),
            "NP" | "NounPhrase" => Ok(ConstituentLabel::NounPhrase),
            "VP" | "VerbPhrase" => Ok(ConstituentLabel::VerbPhrase),
            "PP" | "PrepositionalPhrase" => Ok(ConstituentLabel::PrepositionalPhrase),
            "N" | "Noun" => Ok(ConstituentLabel::Noun),
            "V" | "Verb" => Ok(ConstituentLabel::Verb),
            "ADJ" | "Adj" | "Adjective" => Ok(ConstituentLabel::Adjective),
            "ADV" | "Adv" | "Adverb" => Ok(ConstituentLabel::Adverb),
            "DET" | "Det" | "Determiner" => Ok(ConstituentLabel::Determiner),
            "P" | "Preposition" => Ok(ConstituentLabel::Preposition),
            "CONJ" | "Conj" | "Conjunction" => Ok(ConstituentLabel::Conjunction),
            "SCONJ" | "Sconj" | "SubordinatingConjunction" => {
                Ok(ConstituentLabel::SubordinatingConjunction)
            }
            "COMP" | "Comp" | "Complementizer" => Ok(ConstituentLabel::Complementizer),
            "INTERJ" | "Interj" | "Interjection" => Ok(ConstituentLabel::Interjection),
            "AUX" | "Aux" | "AuxiliaryVerb" => Ok(ConstituentLabel::AuxiliaryVerb),
            "RELC" | "RelC" | "RelativeClause" => Ok(ConstituentLabel::RelativeClause),
            "INFP" | "InfP" | "InfinitivePhrase" => Ok(ConstituentLabel::InfinitivePhrase),
            "GERP" | "GerP" | "GerundPhrase" => Ok(ConstituentLabel::GerundPhrase),
            "PARTP" | "PartP" | "ParticiplePhrase" => Ok(ConstituentLabel::ParticiplePhrase),
            "Q" | "Question" => Ok(ConstituentLabel::Question),
            _ => Err(()),
        }
    }
}

impl<'py> FromPyObject<'py> for ConstituentLabel {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: &str = ob.extract()?;
        Self::try_from(s)
            .map_err(|_| PyErr::new::<PyValueError, _>(format!("Invalid constituent label: {}", s)))
    }
}
