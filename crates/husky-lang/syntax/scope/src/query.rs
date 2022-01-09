use crate::{error::scope_err, *};

use common::*;

use word::Word;

use fold::FoldStorage;

use std::{path::PathBuf, sync::Arc};
#[salsa::query_group(ScopeQueryStorage)]
pub trait ScopeSalsaQuery: token::TokenSalsaQuery + InternScope {
    fn subscope_table(&self, scope_id: ScopeId) -> ScopeResultArc<SubscopeTable>;

    fn subscope_ids(&self, scope_id: ScopeId) -> Arc<Vec<ScopeId>>;

    fn scope_kind(&self, scope_id: ScopeId) -> ScopeKind;

    fn scope_kind_from_route(&self, route: ScopeRoute) -> ScopeKind;

    fn scope_source(&self, scope_id: ScopeId) -> ScopeResult<ScopeSource>;
}

fn subscope_table(this: &dyn ScopeSalsaQuery, scope_id: ScopeId) -> ScopeResultArc<SubscopeTable> {
    Ok(Arc::new(match this.scope_source(scope_id)? {
        ScopeSource::Builtin(_) => todo!(),
        ScopeSource::WithinCustomModule {
            file_id,
            token_group_index,
        } => {
            let text = this.tokenized_text(file_id)?;
            let item = text.fold_iter(token_group_index).next().unwrap();
            if let Some(children) = item.children {
                SubscopeTable::parse(file_id, children)
            } else {
                SubscopeTable::empty()
            }
        }
        ScopeSource::Module { file_id } => {
            let text = this.tokenized_text(file_id)?;
            SubscopeTable::parse(file_id, text.fold_iter(0))
        }
        ScopeSource::WithinBuiltinModule => todo!(),
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
            | BuiltinIdentifier::Vector
            | BuiltinIdentifier::Tuple
            | BuiltinIdentifier::Fp
            | BuiltinIdentifier::Array => ScopeKind::Type,
            BuiltinIdentifier::Fn | BuiltinIdentifier::FnMut | BuiltinIdentifier::FnOnce => {
                ScopeKind::Trait
            }
            BuiltinIdentifier::Debug | BuiltinIdentifier::Std | BuiltinIdentifier::Core => {
                ScopeKind::Module
            }
            BuiltinIdentifier::Input => ScopeKind::Feature,
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
            | BuiltinIdentifier::Vector
            | BuiltinIdentifier::Tuple
            | BuiltinIdentifier::Fp
            | BuiltinIdentifier::Array => ScopeKind::Type,
            BuiltinIdentifier::Fn | BuiltinIdentifier::FnMut | BuiltinIdentifier::FnOnce => {
                ScopeKind::Trait
            }
            BuiltinIdentifier::Debug | BuiltinIdentifier::Std | BuiltinIdentifier::Core => {
                ScopeKind::Module
            }
            BuiltinIdentifier::Input => ScopeKind::Feature,
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
        generics: Vec<GenericArgument>,
    ) -> Option<Scope> {
        if self
            .subscope_table(parent_scope)
            .map_or(false, |table| table.has_subscope(ident, &generics))
        {
            Some(Scope::child_scope(parent_scope, ident, generics))
        } else {
            None
        }
    }

    fn all_modules(&self) -> Vec<PackageOrModule>
    where
        Self: Sized,
    {
        self.all_main_files()
            .iter()
            .map(|id| self.collect_modules(*id))
            .flatten()
            .collect()
    }

    fn module_iter(&self) -> std::vec::IntoIter<PackageOrModule>
    where
        Self: Sized,
    {
        self.all_modules().into_iter()
    }

    fn collect_modules(&self, id: FileId) -> Vec<PackageOrModule>
    where
        Self: Sized,
    {
        if let Ok(module) = self.module_from_file_id(id) {
            let mut modules = vec![module];
            self.subscope_table(module.scope_id()).ok().map(|table| {
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

    fn module_from_file_id(&self, id: FileId) -> ScopeResult<PackageOrModule> {
        let path: PathBuf = file::convert_filepath(self, id, |pth| pth.into());
        if !self.file_exists(id) {
            scope_err!(format!("file didn't exist"))?
        } else if path_has_file_name(&path, "main.hsk") {
            if let Some(package_name) = path_parent_file_name_str(&path) {
                let word = self.string_to_word(package_name.as_ref());
                if let Word::Identifier(Identifier::Custom(ident)) =
                    self.string_to_word(package_name.as_ref())
                {
                    Ok(PackageOrModule {
                        scope_id: self.intern_scope(Scope::package(id, ident)),
                    })
                } else {
                    scope_err!(format!("package name should be identifier"))?
                }
            } else {
                scope_err!(format!("package root should have filename"))?
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
            scope_err!(format!("file should have extension .hsk"))?
        }
    }

    fn module_to_file_id(&self, module: PackageOrModule) -> ScopeResult<FileId> {
        Ok(match self.scope_source(module.scope_id())? {
            ScopeSource::Builtin(_) => todo!(),
            ScopeSource::WithinCustomModule { file_id, .. } => file_id,
            ScopeSource::Module { file_id } => file_id,
            ScopeSource::WithinBuiltinModule => todo!(),
        })
    }

    fn submodule_file_id(&self, parent_id: FileId, ident: CustomIdentifier) -> ScopeResult<FileId>
    where
        Self: Sized,
    {
        let path = self.filepath(parent_id);

        should!(path_has_file_name(&path, "mod.hsk") || path_has_file_name(&path, "main.hsk"));

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PackageOrModule {
    scope_id: ScopeId,
}

impl PackageOrModule {
    pub fn scope_id(&self) -> ScopeId {
        self.scope_id
    }
}
