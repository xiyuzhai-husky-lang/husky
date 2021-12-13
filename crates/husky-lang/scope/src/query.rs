use crate::*;

use file::FileId;
use word::Word;

use std::sync::Arc;
#[salsa::query_group(ScopeQueryStorage)]
pub trait ScopeSalsaQuery: token::TokenQuery + InternScope {
    fn subscopes(&self, scope_id: ScopeId) -> Arc<ScopeTable>;

    fn scope_kind(&self, scope_id: ScopeId) -> Option<ScopeKind>;

    fn scope_source(&self, scope_id: ScopeId) -> Option<ScopeSource>;
}

fn subscopes(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Arc<ScopeTable> {
    Arc::new(if let Some(source) = this.scope_source(scope_id) {
        match source {
            ScopeSource::Builtin(_) => todo!(),
            ScopeSource::File {
                file_id,
                token_group_index,
            } => {
                if let Some(text) = this.tokenized_text(file_id).as_ref().as_ref().ok() {
                    if let Some(index) = token_group_index {
                        if let Some(children) = text.folded_iter(index).children() {
                            ScopeTable::parse(file_id, children)
                        } else {
                            ScopeTable::default()
                        }
                    } else {
                        ScopeTable::parse(file_id, text.folded_iter(0))
                    }
                } else {
                    ScopeTable::default()
                }
            }
        }
    } else {
        ScopeTable::default()
    })
}

fn scope_kind(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Option<ScopeKind> {
    let scope = this.id_to_scope(scope_id);
    match scope.parent {
        ScopeParent::Scope(parent) => this.subscopes(parent).scope_kind(scope.ident),
        ScopeParent::Package(_) => todo!(),
        ScopeParent::Root => todo!(),
    }
}

fn scope_source(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Option<ScopeSource> {
    let scope = this.id_to_scope(scope_id);
    match scope.parent {
        ScopeParent::Scope(parent) => this.subscopes(parent).scope_source(scope.ident),
        ScopeParent::Package(main_file_id) => Some(ScopeSource::File {
            file_id: main_file_id,
            token_group_index: None,
        }),
        ScopeParent::Root => todo!(),
    }
}

pub trait ScopeQuery: ScopeSalsaQuery + InternScope {
    fn subscope(&self, scope: ScopeKind, word: Word) -> ScopeKind {
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
            modules.extend(
                self.subscopes(module.scope_id)
                    .submodules()
                    .into_iter()
                    .map(|ident| self.collect_modules(self.submodule_file_id(id, ident)))
                    .flatten(),
            );
            modules
        } else {
            vec![]
        }
    }

    fn module_from_file_id(&self, id: FileId) -> Option<Module> {
        let path = self.file_path(id);
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
        let path = self.file_path(id);
        todo!()
    }
}
