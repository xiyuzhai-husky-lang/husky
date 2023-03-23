#[derive(Debug)]
pub enum Formula {
    Integral,
    Matrix(MatrixFormula),
}

#[derive(Debug)]
pub struct MatrixFormula {
    shape: (u8, u8),
    data: MatrixFormulaData,
}

impl MatrixFormula {
    pub fn shape(&self) -> (u8, u8) {
        self.shape
    }

    pub fn data(&self) -> &MatrixFormulaData {
        &self.data
    }
}

#[derive(Debug)]
pub enum MatrixFormulaData {
    Scattered { entries: Vec<(u8, u8, Formula)> },
}
