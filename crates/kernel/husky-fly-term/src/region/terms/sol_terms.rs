use super::*;
use vec_like::VecSet;

// `Default` is not implemented because we might need to initialize it from the parent
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SolTerms {
    entries: VecSet<SolidTermData>,
}

impl SolTerms {
    pub(crate) fn new(terms: Option<&SolTerms>) -> Self {
        let entries = match terms {
            Some(terms) => terms.entries.clone(),
            None => Default::default(),
        };
        Self { entries }
    }

    pub(crate) fn intern_new(&mut self, data: SolidTermData) -> SolTerm {
        let raw = self
            .entries
            .position_or_insert(data)
            .try_into()
            .expect("size of entries shouldn't be too large");
        assert!((raw as usize) < self.entries.len());
        SolTerm(raw)
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolTerm(u32);

impl SolTerm {
    pub(crate) fn new(solid_terms: &mut SolTerms, data: SolidTermData) -> Self {
        solid_terms.intern_new(data)
    }

    pub(crate) fn data(self, engine: &impl FlyTermEngine) -> &SolidTermData {
        self.data_inner(&engine.fluffy_terms().solid_terms())
    }

    pub(crate) fn data_inner(self, solid_terms: &SolTerms) -> &SolidTermData {
        &solid_terms.entries.data()[self.0 as usize]
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db, solid_terms: &SolTerms) -> String {
        match self.data_inner(solid_terms) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolidTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// #[salsa::derive_debug_with_db(db = FlyTermDb)]
pub enum SolidTermSource {}
