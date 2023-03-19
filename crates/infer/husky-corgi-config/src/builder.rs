use crate::*;

pub(crate) struct CorgiConfigBuilder<'a> {
    db: &'a dyn CorgiConfigDb,
    registry_path: Option<DiffPath>,
    errors: Vec<CorgiConfigError>,
}

impl<'a> CorgiConfigBuilder<'a> {
    pub(crate) fn new(db: &'a dyn CorgiConfigDb) -> Self {
        Self {
            db,
            registry_path: Default::default(),
            errors: Default::default(),
        }
    }

    pub(crate) fn read(&mut self, path: DiffPath) -> VfsResult<()> {
        let corgi_config_ast_sheet: &CorgiConfigAstSheet = self.db.corgi_config_ast_sheet(path)?;
        todo!()
    }

    pub(crate) fn finish(self) -> CorgiConfig {
        CorgiConfig {
            registry_section: RegistrySection {
                path: self.registry_path.unwrap_or_else(|| todo!()),
            },
            errors: todo!(),
        }
    }
}
