#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembSignature {
    pub kind: MembSignatureKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MembSignatureKind {
    Var,
    Routine,
}
