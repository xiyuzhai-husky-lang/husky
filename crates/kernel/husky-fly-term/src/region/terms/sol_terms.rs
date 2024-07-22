use super::*;
use vec_like::VecSet;

// `Default` is not implemented because we might need to initialize it from the parent
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SolTerms {
    entries: VecSet<SolTermData>,
}

impl SolTerms {
    pub(crate) fn new(terms: Option<&SolTerms>) -> Self {
        let entries = match terms {
            Some(terms) => terms.entries.clone(),
            None => Default::default(),
        };
        Self { entries }
    }

    pub(crate) fn intern_new(&mut self, data: SolTermData) -> SolTerm {
        let raw = self
            .entries
            .position_or_insert(data)
            .try_into()
            .expect("size of entries shouldn't be too large");
        assert!((raw as usize) < self.entries.len());
        SolTerm(raw)
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SolTerm(u32);

impl SolTerm {
    pub(crate) fn new(sol_terms: &mut SolTerms, data: SolTermData) -> Self {
        sol_terms.intern_new(data)
    }

    pub fn data(self, engine: &impl FlyTermEngine) -> &SolTermData {
        self.data2(&engine.fly_terms().sol_terms())
    }

    pub fn data2(self, sol_terms: &SolTerms) -> &SolTermData {
        &sol_terms.entries.data()[self.0 as usize]
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db, sol_terms: &SolTerms) -> String {
        match self.data2(sol_terms) {
            SolTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolTermData::Curry {
                toolchain,
                curry_kind,
                variance,
                parameter_hvar,
                parameter_ty,
                return_ty,
            } => todo!(),
            SolTermData::Ritchie {
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
