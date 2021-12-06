//! This module specifies the input to husky-lang-server. In some sense, this is
//! **the** most important module, because all other fancy stuff is strictly
//! derived from this input.
//!
//! Note that neither this module, nor any other part of the analyzer's core do
//! actual IO. See `vfs` and `project_model` in the `husky-lang-server` crate for how
//! actual IO is done and lowered to input.

use std::{fmt, iter::FromIterator, ops, panic::RefUnwindSafe, str::FromStr, sync::Arc};

use rustc_hash::{FxHashMap, FxHashSet};
use syntax::SmolStr;
use vfs::{file_set::FilePathIdTable, FileID, VfsPath};

/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes. Typically it corresponds to a
/// Rust crate. Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root. Paths to files are always relative to a
/// source root, and the analyzer does not know the root path of the source root at
/// all. So, a file from one source root can't refer to a file in another source
/// root by path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    /// Sysroot or crates.io library.
    ///
    /// Libraries are considered mostly immutable, this assumption is used to
    /// optimize salsa's query structure
    pub is_library: bool,
    pub(crate) file_set: FilePathIdTable,
}

impl SourceRoot {
    pub fn new_local(file_set: FilePathIdTable) -> SourceRoot {
        SourceRoot {
            is_library: false,
            file_set,
        }
    }
    pub fn new_library(file_set: FilePathIdTable) -> SourceRoot {
        SourceRoot {
            is_library: true,
            file_set,
        }
    }
    pub fn path_for_file(&self, file: &FileID) -> Option<&VfsPath> {
        self.file_set.get_path_for_id(file)
    }
    pub fn file_for_path(&self, path: &VfsPath) -> Option<FileID> {
        self.file_set.file_for_path(path)
    }
    pub fn iter(&self) -> impl Iterator<Item = FileID> + '_ {
        self.file_set.id_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrateId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateName(SmolStr);

impl CrateName {
    /// Creates a crate name, checking for dashes in the string provided.
    /// Dashes are not allowed in the crate names,
    /// hence the input string is returned as `Err` for those cases.
    pub fn new(name: &str) -> Result<CrateName, &str> {
        if name.contains('-') {
            Err(name)
        } else {
            Ok(Self(SmolStr::new(name)))
        }
    }

    /// Creates a crate name, unconditionally replacing the dashes with underscores.
    pub fn normalize_dashes(name: &str) -> CrateName {
        Self(SmolStr::new(name.replace('-', "_")))
    }
}

impl fmt::Display for CrateName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ops::Deref for CrateName {
    type Target = str;
    fn deref(&self) -> &str {
        &*self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateDisplayName {
    // The name we use to display various paths (with `_`).
    crate_name: CrateName,
    // The name as specified in Cargo.toml (with `-`).
    canonical_name: String,
}

impl CrateDisplayName {
    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }
    pub fn crate_name(&self) -> &CrateName {
        &self.crate_name
    }
}

impl From<CrateName> for CrateDisplayName {
    fn from(crate_name: CrateName) -> CrateDisplayName {
        let canonical_name = crate_name.to_string();
        CrateDisplayName {
            crate_name,
            canonical_name,
        }
    }
}

impl fmt::Display for CrateDisplayName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.crate_name)
    }
}

impl ops::Deref for CrateDisplayName {
    type Target = str;
    fn deref(&self) -> &str {
        &*self.crate_name
    }
}

impl CrateDisplayName {
    pub fn from_canonical_name(canonical_name: String) -> CrateDisplayName {
        let crate_name = CrateName::normalize_dashes(&canonical_name);
        CrateDisplayName {
            crate_name,
            canonical_name,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ProcMacroId(pub u32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ProcMacroKind {
    CustomDerive,
    FuncLike,
    Attr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Edition {
    Edition2015,
    Edition2018,
    Edition2021,
}

impl Edition {
    pub const CURRENT: Edition = Edition::Edition2018;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Env {
    entries: FxHashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub crate_id: CrateId,
    pub name: CrateName,
    prelude: bool,
}

impl Dependency {
    pub fn new(name: CrateName, crate_id: CrateId) -> Self {
        Self {
            name,
            crate_id,
            prelude: true,
        }
    }

    pub fn with_prelude(name: CrateName, crate_id: CrateId, prelude: bool) -> Self {
        Self {
            name,
            crate_id,
            prelude,
        }
    }

    /// Whether this dependency is to be added to the depending crate's extern prelude.
    pub fn is_prelude(&self) -> bool {
        self.prelude
    }
}

impl FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "2015" => Edition::Edition2015,
            "2018" => Edition::Edition2018,
            "2021" => Edition::Edition2021,
            _ => {
                return Err(ParseEditionError {
                    invalid_input: s.to_string(),
                })
            }
        };
        Ok(res)
    }
}

impl fmt::Display for Edition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Edition::Edition2015 => "2015",
            Edition::Edition2018 => "2018",
            Edition::Edition2021 => "2021",
        })
    }
}

impl FromIterator<(String, String)> for Env {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Env {
            entries: FromIterator::from_iter(iter),
        }
    }
}

impl Env {
    pub fn set(&mut self, env: &str, value: String) {
        self.entries.insert(env.to_owned(), value);
    }

    pub fn get(&self, env: &str) -> Option<String> {
        self.entries.get(env).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

#[derive(Debug)]
pub struct ParseEditionError {
    invalid_input: String,
}

impl fmt::Display for ParseEditionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid edition: {:?}", self.invalid_input)
    }
}

impl std::error::Error for ParseEditionError {}

#[derive(Debug)]
pub struct CyclicDependenciesError {
    path: Vec<(CrateId, Option<CrateDisplayName>)>,
}

impl CyclicDependenciesError {
    fn from(&self) -> &(CrateId, Option<CrateDisplayName>) {
        self.path.first().unwrap()
    }
    fn to(&self) -> &(CrateId, Option<CrateDisplayName>) {
        self.path.last().unwrap()
    }
}

impl fmt::Display for CyclicDependenciesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let render = |(id, name): &(CrateId, Option<CrateDisplayName>)| match name {
            Some(it) => format!("{}({:?})", it, id),
            None => format!("{:?}", id),
        };
        let path = self
            .path
            .iter()
            .rev()
            .map(render)
            .collect::<Vec<String>>()
            .join(" -> ");
        write!(
            f,
            "cyclic deps: {} -> {}, alternative path: {}",
            render(self.from()),
            render(self.to()),
            path
        )
    }
}
