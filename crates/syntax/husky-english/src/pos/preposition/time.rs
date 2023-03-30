use super::*;
use TimePreposition::*;

/// time preposition in English
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TimePreposition {
    /// specific time of day
    At,
    In,
    /// specific date
    On,
    For,
    During,
    Since,
    By,
    Util,
    Before,
    After,
    To,
    Past,
}

impl TimePreposition {
    pub fn to_str(self) -> &'static str {
        match self {
            At => "at",
            In => "in",
            On => "on",
            For => "for",
            During => "during",
            Since => "since",
            By => "by",
            Util => "util",
            Before => "before",
            After => "after",
            To => "to",
            Past => "past",
        }
    }
}
