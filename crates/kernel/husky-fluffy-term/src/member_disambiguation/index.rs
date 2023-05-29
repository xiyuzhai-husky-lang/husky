use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyIndexSignature {}

impl FluffyTerm {
    pub fn index_signature(
        self,
        engine: &mut impl FluffyTermEngine,
        indice_tys: &[FluffyTerm],
    ) -> FluffyTermMaybeResult<FluffyIndexSignature> {
        todo!()
    }
}
