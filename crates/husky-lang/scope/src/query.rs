use crate::*;

use file::{FileId, FileResultArc};
use word::Word;

use std::{path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryStorage)]
pub trait ScopeSalsaQuery: token::TokenQuery + InternScope {
    fn subscope_table(&self, scope_id: ScopeId) -> FileResultArc<SubscopeTable>;

    fn subscope_ids(&self, scope_id: ScopeId) -> Arc<Vec<ScopeId>>;

    fn scope_kind(&self, scope_id: ScopeId) -> Option<ScopeKind>;

    fn scope_source(&self, scope_id: ScopeId) -> Option<ScopeSource>;
}

fn subscope_table(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> FileResultArc<SubscopeTable> {
    if let Some(source) = this.scope_source(scope_id) {
        match source {
            ScopeSource::Builtin(_) => todo!(),
            ScopeSource::WithinModule {
                file_id,
                token_group_index,
            } => this.tokenized_text(file_id).map(|text| {
                if let Some(children) = text.folded_iter(token_group_index).children() {
                    Arc::new(SubscopeTable::parse(file_id, children))
                } else {
                    Arc::new(SubscopeTable::empty())
                }
            }),
            ScopeSource::Module { file_id } => this
                .tokenized_text(file_id)
                .map(|text| Arc::new(SubscopeTable::parse(file_id, text.folded_iter(0)))),
        }
    } else {
        todo!()
        // ScopeTable::default()
    }
}

fn subscope_ids(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Arc<Vec<ScopeId>> {
    Arc::new(if let Some(table) = this.subscope_table(scope_id).ok() {
        table
            .subscopes(scope_id)
            .into_iter()
            .map(|scope| this.scope_to_id(scope))
            .collect()
    } else {
        Vec::new()
    })
}

fn scope_kind(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Option<ScopeKind> {
    let scope = this.id_to_scope(scope_id);
    match scope.parent {
        ScopeParent::Scope(parent) => this
            .subscope_table(parent)
            .as_ref()
            .as_ref()
            .ok()
            .map(|table| table.scope_kind(scope.ident))
            .flatten(),
        ScopeParent::Package(_) => todo!(),
        ScopeParent::Root => todo!(),
    }
}

fn scope_source(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Option<ScopeSource> {
    let scope = this.id_to_scope(scope_id);
    match scope.parent {
        ScopeParent::Scope(parent) => this
            .subscope_table(parent)
            .as_ref()
            .as_ref()
            .ok()
            .map(|table| table.scope_source(scope.ident))
            .flatten(),
        ScopeParent::Package(main_file_id) => Some(ScopeSource::Module {
            file_id: main_file_id,
        }),
        ScopeParent::Root => todo!(),
    }
}

pub trait ScopeQuery: ScopeSalsaQuery + InternScope {
    fn subscope(&self, scope: ScopeId, word: Word) -> ScopeId {
        todo!()
    }

    fn all_modules(&self) -> Vec<Module> {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn collect_modules(&self, id: FileId) -> Vec<Module> {
        if let Some(module) = self.module_from_file_id(id) {
            let mut modules = vec![module];
            self.subscope_table(module.scope_id).ok().map(|table| {
                modules.extend(
                    table
                        .submodules()
                        .into_iter()
                        .map(|ident| self.collect_modules(self.submodule_file_id(id, ident)))
                        .flatten(),
                )
            });
            modules
        } else {
            vec![]
        }
    }

    fn module_from_file_id(&self, id: FileId) -> Option<Module> {
        let path: PathBuf = file::use_filepath(self, id, |pth| pth.into());
        if path.file_name().map(|s| s.to_string_lossy()) == Some("main.hsk".into()) {
            if let Some(package_root) = path.parent() {
                if let Some(package_name) = package_root.file_name().map(|s| s.to_string_lossy()) {
                    match self.string_to_word(package_name.as_ref()) {
                        Word::Keyword(_) => return None,
                        Word::Identifier(ident) => {
                            let scope = Scope {
                                ident,
                                parent: ScopeParent::Package(id),
                            };
                            return Some(Module {
                                scope_id: self.scope_to_id(scope),
                            });
                        }
                    }
                }
            }
            return None;
        }
        if path.extension().map(|s| s.to_string_lossy()) == Some("hsk".into())
            && self.file_exists(id)
        {
            let parent = self.module_from_file_id(self.file_id(path.with_file_name("mod.hsk")));
            todo!()
            // return Some(Module(id));
        } else {
            return None;
        }
    }

    fn submodule_file_id(&self, id: FileId, ident: Identifier) -> FileId {
        // let path = self.file_id_to_path(id);
        todo!()
    }
}
