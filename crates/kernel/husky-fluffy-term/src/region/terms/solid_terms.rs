use super::*;
use vec_like::VecSet;

// `Default` is not implemented because we might need to initialize it from the parent
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SolidTerms {
    entries: VecSet<SolidTermData>,
}

impl SolidTerms {
    pub(crate) fn new(terms: Option<&SolidTerms>) -> Self {
        let entries = match terms {
            Some(terms) => terms.entries.clone(),
            None => Default::default(),
        };
        Self { entries }
    }

    pub(crate) fn intern_new(&mut self, data: SolidTermData) -> SolidTerm {
        let raw = self
            .entries
            .position_or_insert(data)
            .try_into()
            .expect("size of entries shouldn't be too large");
        assert!((raw as usize) < self.entries.len());
        SolidTerm(raw)
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolidTerm(u32);

impl SolidTerm {
    pub(crate) fn new(solid_terms: &mut SolidTerms, data: SolidTermData) -> Self {
        solid_terms.intern_new(data)
    }

    pub(crate) fn data(self, engine: &impl FlyTermEngine) -> &SolidTermData {
        self.data_inner(&engine.fluffy_terms().solid_terms())
    }

    pub(crate) fn data_inner(self, solid_terms: &SolidTerms) -> &SolidTermData {
        &solid_terms.entries.data()[self.0 as usize]
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db, solid_terms: &SolidTerms) -> String {
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
