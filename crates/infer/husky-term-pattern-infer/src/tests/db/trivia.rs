use super::*;

impl Upcast<dyn TermDb> for TermPatternInferFakeDb {
    fn upcast(&self) -> &(dyn TermDb + 'static) {
        self
    }
}

impl Database for TermPatternInferFakeDb {}
