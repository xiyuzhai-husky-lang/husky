mod leashed;
mod memo;
mod regular;

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyFieldDisambiguation {
    Field,
    Memo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldDisambiguationVariant {
    Field,
    Memo,
}
