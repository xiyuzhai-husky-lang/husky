use super::*;

pub enum VdSynSentenceChild {
    Clause(VdSynClauseIdx),
}

impl VdSynSentenceData {
    pub(crate) fn children(&self) -> Vec<VdSynSentenceChild> {
        match self {
            VdSynSentenceData::Clauses { clauses, .. } => clauses
                .into_iter()
                .map(VdSynSentenceChild::Clause)
                .collect(),
        }
    }
}
