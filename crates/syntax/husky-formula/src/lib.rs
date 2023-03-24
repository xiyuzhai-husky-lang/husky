#[derive(Debug)]
#[enum_class::from_variants]
pub enum Formula {
    Integral,
    Matrix(MatrixFormula),
    Literal(LiteralFormula),
}

#[derive(Debug)]
pub enum LiteralFormula {
    I32(i32),
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
#[enum_class::from_variants]
pub enum MatrixFormulaData {
    Scattered(Vec<(u8, u8, Formula)>),
}

pub fn new_ad_hoc_matrix() -> Formula {
    Formula::Matrix(MatrixFormula {
        shape: (3, 3),
        data: vec![
            (0, 0, LiteralFormula::I32(1).into()),
            (0, 0, LiteralFormula::I32(1).into()),
            (0, 0, LiteralFormula::I32(1).into()),
        ]
        .into(),
    })
}
