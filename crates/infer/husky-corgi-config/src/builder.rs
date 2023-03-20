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

    pub(crate) fn read(&mut self, path: DiffPath) -> CorgiConfigResult<()> {
        let Some(corgi_config_ast_sheet) = self.db.corgi_config_ast_sheet(path)? else {
            return Ok(())
        };
        if let Some(registry_section) = corgi_config_ast_sheet.registry_section() {
            let registry_section = registry_section?;
            if self.registry_path.is_none() && let Some(path) = registry_section.path() {
                let path = path?;
                todo!()
            }
        }
        Ok(())
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
