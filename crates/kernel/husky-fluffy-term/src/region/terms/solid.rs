use super::*;
use salsa::DisplayWithDb;
use vec_like::{AsVecMapEntry, VecMap, VecSet};

// `Default` is not implemented because we might need to initialize it from the parent
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
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

    fn intern(&mut self, data: SolidTermData) -> SolidTerm {
        let raw = self
            .entries
            .position_or_insert(data)
            .try_into()
            .expect("size of entries shouldn't be too large");
        assert!((raw as usize) < self.entries.len());
        SolidTerm(raw)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct SolidTerm(u32);

impl SolidTerm {
    pub(crate) fn new(solid_terms: &mut SolidTerms, data: SolidTermData) -> Self {
        solid_terms.intern(data)
    }

    pub(crate) fn data(self, engine: &impl FluffyTermEngine) -> &SolidTermData {
        self.data_inner(&engine.fluffy_terms().solid_terms())
    }

    pub(crate) fn data_inner(self, solid_terms: &SolidTerms) -> &SolidTermData {
        &solid_terms.entries.data()[self.0 as usize]
    }

    pub fn show(self, db: &dyn FluffyTermDb, solid_terms: &SolidTerms) -> String {
        match self.data_inner(solid_terms) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            // SolidTermData::TypeOntologyAtPlace {
            //     ty_path: path,
            //     refined_ty_path: refined_path,
            //     arguments,
            //     base_ty_term,
            //     place,
            // } => match base_ty_term {
            //     Some(base_ty_term) => format!("@{:?} {}", place, base_ty_term.display(db)),
            //     None => todo!(),
            // },
            SolidTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
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
// #[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum SolidTermSource {}
