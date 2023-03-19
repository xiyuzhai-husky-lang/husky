use crate::*;

#[derive(Debug, Default)]
pub(crate) struct CorgiConfigBuilder {
    errors: Vec<CorgiConfigError>,
}

impl CorgiConfigBuilder {
    fn read(&mut self, ast_sheet: &CorgiConfigAstSheet) -> CorgiConfig {
        todo!()
    }

    pub(crate) fn finish(self) -> CorgiConfig {
        CorgiConfig {}
    }
}
