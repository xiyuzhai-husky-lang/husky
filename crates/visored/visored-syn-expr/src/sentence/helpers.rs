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
            VdSynSentenceData::Have => todo!(),
            VdSynSentenceData::Show => todo!(),
            VdSynSentenceData::Let {
                left_math_delimiter_token_idx,
                formula,
                right_math_delimiter_token_idx,
            } => todo!(),
        }
    }
}
