use super::*;

impl<'sess> VdBsqExprFld<'sess> {
    pub fn is_zero(self) -> bool {
        self.eqs_nat128(0)
    }

    pub fn is_one(self) -> bool {
        self.eqs_nat128(1)
    }

    pub fn eqs_nat128(self, i: i128) -> bool {
        let VdBsqExprFldData::Literal(lit) = self.data() else {
            return false;
        };
        let VdLiteralData::Int128(i1) = *lit.data() else {
            return false;
        };
        i1 == i
    }
}
