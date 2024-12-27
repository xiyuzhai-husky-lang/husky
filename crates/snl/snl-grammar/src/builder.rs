use std::path::Path;

use crate::{
    error::SnlGrammarResult, grammar::SnlGrammar, module::SnlModuleData,
    rewite_rule::SnlRewriteRuleData,
};
use eterned::db::EternerDb;
use snl_prelude::{
    coword::SnlIdent,
    path::{
        module::{SnlModulePath, SnlModulePathData},
        rewrite_rule::SnlRewriteRulePath,
    },
};
use vec_like::ordered_vec_map::OrderedVecPairMap;

pub(crate) struct SnlGrammarBuilder<'db> {
    db: &'db EternerDb,
    modules: OrderedVecPairMap<SnlModulePath, SnlModuleData>,
    rewrite_rules: OrderedVecPairMap<SnlRewriteRulePath, SnlRewriteRuleData>,
}

impl<'db> SnlGrammarBuilder<'db> {
    pub fn new(db: &'db EternerDb) -> Self {
        Self {
            db,
            modules: Default::default(),
            rewrite_rules: Default::default(),
        }
    }
}

impl<'db> SnlGrammarBuilder<'db> {
    pub fn scan_root(&mut self, dir: impl AsRef<Path>) -> SnlGrammarResult<()> {
        let root = SnlModulePath::new(SnlModulePathData::Root, self.db);
        self.scan_dir(root, dir)
    }

    fn scan_dir(&mut self, module: SnlModulePath, dir: impl AsRef<Path>) -> SnlGrammarResult<()> {
        let dir = dir.as_ref();
        let entries = std::fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.exists() && path.is_file() && path.extension() == Some("snl.yaml".as_ref()) {
                let file_stem = path.file_stem().unwrap();
                let ident = SnlIdent::from_ref(file_stem.to_str().unwrap(), self.db)?;
                let module_path = SnlModulePath::new(
                    SnlModulePathData::Child {
                        parent: todo!(),
                        ident,
                    },
                    self.db,
                );
                let possible_submodules_dir = path.with_extension("");
                if possible_submodules_dir.exists() {
                    if !possible_submodules_dir.is_dir() {
                        todo!()
                    }
                    self.scan_nonterminal_module_file(module_path, path)?;
                    self.scan_dir(module_path, possible_submodules_dir)?;
                } else {
                    self.scan_terminal_module_file(module_path, path)?;
                }
            }
        }
        Ok(())
    }

    fn scan_nonterminal_module_file(
        &mut self,
        module_path: SnlModulePath,
        path: impl AsRef<Path>,
    ) -> SnlGrammarResult<()> {
        let path = path.as_ref();
        let file_content = std::fs::read_to_string(path)?;
        let module_data = SnlModuleData::Nonterminal(todo!());
        self.modules.insert_new((module_path, module_data)).unwrap();
        Ok(())
    }

    fn scan_terminal_module_file(
        &mut self,
        module_path: SnlModulePath,
        path: impl AsRef<Path>,
    ) -> SnlGrammarResult<()> {
        let path = path.as_ref();
        let file_content = std::fs::read_to_string(path)?;
        let module_data = SnlModuleData::Terminal(todo!());
        self.modules.insert_new((module_path, module_data)).unwrap();
        Ok(())
    }

    pub fn finish(self) -> SnlGrammar {
        SnlGrammar::new(self.modules, self.rewrite_rules)
    }
}
