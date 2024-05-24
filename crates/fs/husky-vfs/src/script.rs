use crate::*;

#[salsa::input(db = VfsDb, jar = VfsJar)]
pub struct Script {
    pub source: ScriptSource,
    #[return_ref]
    pub data: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptSource {
    Snippet { toolchain: Toolchain },
    Child { parent: Script },
}

impl Script {
    #[cfg(feature = "test_utils")]
    pub fn new_dev_snippet(data: impl Into<String>, db: &::salsa::Db) -> Self {
        let toolchain = db.dev_toolchain().unwrap();
        Self::new(db, ScriptSource::Snippet { toolchain }, data.into())
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        script_toolchain(db, self)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ScriptModulePath {
        ScriptModulePath::new(self, db)
    }
}

#[salsa::tracked]
fn script_toolchain(db: &::salsa::Db, script: Script) -> Toolchain {
    match script.source(db) {
        ScriptSource::Snippet { toolchain } => todo!(),
        ScriptSource::Child { parent } => todo!(),
    }
}
