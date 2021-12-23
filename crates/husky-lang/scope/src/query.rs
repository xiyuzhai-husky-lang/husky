use crate::*;

use common::*;

use word::Word;

use folded::FoldedStorage;

use std::{path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryStorage)]
pub trait ScopeSalsaQuery: token::TokenSalsaQuery + InternScope {
    fn subscope_table(&self, scope_id: ScopeId) -> ScopeResultArc<SubscopeTable>;

    fn scope_alias_table(&self, scope_id: ScopeId) -> ScopeResultArc<ScopeAliasTable>;

    fn subscope_ids(&self, scope_id: ScopeId) -> Arc<Vec<ScopeId>>;

    fn scope_kind(&self, scope_id: ScopeId) -> ScopeKind;

    fn scope_kind_from_route(&self, route: ScopeRoute) -> ScopeKind;

    fn scope_source(&self, scope_id: ScopeId) -> ScopeResult<ScopeSource>;
}

fn subscope_table(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match this.scope_source(scope_id)? {
        ScopeSource::Builtin(_) => todo!(),
        ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => {
            let text = this.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.folded_iter(token_group_index).children())
        }
        ScopeSource::Module { file_id } => {
            let text = this.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.folded_iter(0))
        }
    }))
}

fn scope_alias_table(
    this: &dyn ScopeSalsaQuery,
    scope_id: ScopeId,
) -> ScopeResultArc<ScopeAliasTable> {
    Ok(Arc::new(match this.scope_source(scope_id)? {
        ScopeSource::Builtin(_) => ScopeAliasTable::empty(),
        ScopeSource::WithinModule {
            file_id,
            token_group_index,
        } => ScopeAliasTable::parse(
            file_id,
            this.tokenized_text(file_id)?
                .folded_iter(token_group_index)
                .children(),
        ),
        ScopeSource::Module { file_id } => {
            ScopeAliasTable::parse(file_id, this.tokenized_text(file_id)?.folded_iter(0))
        }
    }))
}

fn subscope_ids(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> Arc<Vec<ScopeId>> {
    Arc::new(this.subscope_table(scope_id).map_or(Vec::new(), |table| {
        table
            .subscopes(scope_id)
            .into_iter()
            .map(|scope| this.intern_scope(scope))
            .collect()
    }))
}

fn scope_kind(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> ScopeKind {
    let scope = this.id_to_scope(scope_id);
    match scope.route {
        ScopeRoute::Builtin(scope) => match scope {
            BuiltinIdentifier::Void
            | BuiltinIdentifier::I32
            | BuiltinIdentifier::F32
            | BuiltinIdentifier::Vec
            | BuiltinIdentifier::Tuple
            | BuiltinIdentifier::Fp
            | BuiltinIdentifier::Array => ScopeKind::Type,
            BuiltinIdentifier::Fn | BuiltinIdentifier::FnMut | BuiltinIdentifier::FnOnce => {
                ScopeKind::Trait
            }
            BuiltinIdentifier::Debug | BuiltinIdentifier::Std | BuiltinIdentifier::Core => {
                ScopeKind::Module
            }
        },
        ScopeRoute::Package(_, _) => ScopeKind::Module,
        ScopeRoute::ChildScope(parent, ident) => this
            .subscope_table(parent)
            .as_ref()
            .as_ref()
            .ok()
            .map(|table| table.scope_kind(ident))
            .flatten()
            .unwrap(),
    }
}

fn scope_kind_from_route(this: &dyn ScopeSalsaQuery, route: ScopeRoute) -> ScopeKind {
    match route {
        ScopeRoute::Builtin(scope) => match scope {
            BuiltinIdentifier::Void
            | BuiltinIdentifier::I32
            | BuiltinIdentifier::F32
            | BuiltinIdentifier::Vec
            | BuiltinIdentifier::Tuple
            | BuiltinIdentifier::Fp
            | BuiltinIdentifier::Array => ScopeKind::Type,
            BuiltinIdentifier::Fn | BuiltinIdentifier::FnMut | BuiltinIdentifier::FnOnce => {
                ScopeKind::Trait
            }
            BuiltinIdentifier::Debug | BuiltinIdentifier::Std | BuiltinIdentifier::Core => {
                ScopeKind::Module
            }
        },
        ScopeRoute::Package(_, _) => ScopeKind::Module,
        ScopeRoute::ChildScope(parent, ident) => this
            .subscope_table(parent)
            .as_ref()
            .as_ref()
            .ok()
            .map(|table| table.scope_kind(ident))
            .flatten()
            .unwrap(),
    }
}

fn scope_source(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> ScopeResult<ScopeSource> {
    let scope = this.id_to_scope(scope_id);
    Ok(match scope.route {
        ScopeRoute::Builtin(_) => todo!(),
        ScopeRoute::Package(main_file_id, _) => ScopeSource::Module {
            file_id: main_file_id,
        },
        ScopeRoute::ChildScope(parent, ident) => {
            this.subscope_table(parent)?.scope_source(ident)?
        }
    })
}

pub struct ModuleFromFileError {
    pub rule_broken: ModuleFromFileRule,
}

pub enum ModuleFromFileRule {
    PackageNameShouldBeIdentifier,
    PackageRootShouldHaveFileName,
    FileShouldExist,
    FileShouldHaveExtensionHSK,
}

pub trait ScopeQuery: ScopeSalsaQuery + InternScope {
    fn subscope(
        &self,
        parent_scope: ScopeId,
        ident: CustomIdentifier,
        generic_arguments: Vec<GenericArgument>,
    ) -> Option<Scope> {
        if self
            .subscope_table(parent_scope)
            .map_or(false, |table| table.has_subscope(ident, &generic_arguments))
        {
            Some(Scope::child_scope(parent_scope, ident, generic_arguments))
        } else {
            None
        }
    }

    fn all_modules(&self) -> Vec<Module>
    where
        Self: Sized,
    {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<Module>
    where
        Self: Sized,
    {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, id: FileId) -> Vec<Module>
    where
        Self: Sized,
    {
        if let Ok(module) = self.module_from_file_id(id) {
            let mut modules = vec![module];
            self.subscope_table(module.scope_id).ok().map(|table| {
                modules.extend(
                    table
                        .submodule_idents()
                        .into_iter()
                        .filter_map(|ident| {
                            self.submodule_file_id(id, ident)
                                .map_or(None, |id| Some(self.collect_modules(id)))
                        })
                        .flatten(),
                );
            });
            modules
        } else {
            vec![]
        }
    }

    fn module_from_file_id(&self, id: FileId) -> Result<Module, ModuleFromFileError> {
        let path: PathBuf = file::convert_filepath(self, id, |pth| pth.into());
        if !self.file_exists(id) {
            Err(ModuleFromFileError {
                rule_broken: ModuleFromFileRule::FileShouldExist,
            })
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(package_name) = path_parent_file_name_str(&path) {
                let word = self.string_to_word(package_name.as_ref());
                if let Word::Identifier(Identifier::Custom(ident)) =
                    self.string_to_word(package_name.as_ref())
                {
                    Ok(Module {
                        scope_id: self.intern_scope(Scope::package(id, ident)),
                    })
                } else {
                    Err(ModuleFromFileError {
                        rule_broken: ModuleFromFileRule::PackageNameShouldBeIdentifier,
                    })
                }
            } else {
                Err(ModuleFromFileError {
                    rule_broken: ModuleFromFileRule::PackageRootShouldHaveFileName,
                })
            }
        } else if path_has_file_name(&path, "mod.hsk") {
            todo!()
        } else if path_has_extension(&path, "hsk") {
            let maybe_main_path = path.with_file_name("main.hsk");
            if maybe_main_path.exists() {
                let _parent =
                    self.module_from_file_id(self.file_id(path.with_file_name("mod.hsk")));
                todo!()
            } else {
                todo!()
            }
        } else {
            Err(ModuleFromFileError {
                rule_broken: ModuleFromFileRule::FileShouldHaveExtensionHSK,
            })
        }
    }

    fn module_to_file_id(&self, module: Module) -> ScopeResult<FileId> {
        Ok(match self.scope_source(module.scope_id)? {
            ScopeSource::Builtin(_) => todo!(),
            ScopeSource::WithinModule { file_id, .. } => file_id,
            ScopeSource::Module { file_id } => file_id,
        })
    }

    fn submodule_file_id(&self, parent_id: FileId, ident: CustomIdentifier) -> ScopeResult<FileId>
    where
        Self: Sized,
    {
        let path = self.filepath(parent_id);

        assert!(path_has_file_name(&path, "mod.hsk") || path_has_file_name(&path, "main.hsk"));

        let module_path1 = word::convert_ident(self, ident.into(), |s: &str| {
            path.with_file_name(format!("{}.hsk", s))
        });

        let module_path2 = word::convert_ident(self, ident.into(), |s: &str| {
            path.with_file_name(format!("{}/mod.hsk", s))
        });

        let module_path = if module_path1.is_file() && !module_path2.is_file() {
            Ok(module_path1)
        } else if module_path2.is_file() && !module_path1.is_file() {
            Ok(module_path1)
        } else {
            Err(file::FileError::FileNotFound.into())
        };

        module_path.map(|pth| self.file_id(pth))
    }
}
