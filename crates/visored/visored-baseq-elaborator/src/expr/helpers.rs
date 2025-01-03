use super::*;

impl<'sess> VdMirExprFld<'sess> {
    pub fn is_zero(self) -> bool {
        self.eqs_nat128(0)
    }

    pub fn is_one(self) -> bool {
        self.eqs_nat128(1)
    }

    pub fn eqs_nat128(self, n: u128) -> bool {
        let VdMirExprFldData::Literal(lit) = self.data() else {
            return false;
        };
        let VdLiteralData::Nat128(n0) = lit.data() else {
            return false;
        };
        n == *n0
    }
}
