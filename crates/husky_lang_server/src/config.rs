//!
//! We currently get this config from `initialize` LSP request, which is not the
//! best way to do it, but was the simplest thing we could implement.
//!
//! Of particular interest is the `feature_flags` hash map: while other fields
//! configure the server itself, feature flags are passed into analysis, and
//! tweak things like automatic insertion of `()` in completions.

use common::*;

pub(crate) mod huskyfmt;

macro_rules! try_ {
    ($expr:expr) => {
        || -> _ { Some($expr) }()
    };
}
macro_rules! try_or {
    ($expr:expr, $or:expr) => {
        try_!($expr).unwrap_or($or)
    };
}

use std::path::PathBuf;

use husky_lang_db::helpers::{insert_use::InsertUseConfig, SnippetCap};
use ide::{CompletionConfig, HighlightRelatedConfig, Snippet};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{capabilities::completion_item_edit_resolve, line_collection::OffsetEncoding, lsp_ext};

macro_rules! _server_config_data {
    (struct $name:ident {
        $(
            $(#[doc=$doc:literal])*
            $field:ident $(| $alias:ident)*: $ty:ty = $default:expr,
        )*
    }) => {
        #[allow(non_snake_case)]
        #[derive(Debug, Clone)]
        struct $name { $($field: $ty,)* }
        impl $name {
            fn from_json(mut json: serde_json::Value) -> $name {
                $name {$(
                    $field: get_field(
                        &mut json,
                        stringify!($field),
                        None$(.or(Some(stringify!($alias))))*,
                        $default,
                    ),
                )*}
            }
        }
    };
}
use _server_config_data as server_config_data;

// Defines the server-side configuration of the husky-analyzer. We generate
// *parts* of VS Code's `package.json` config from this.
//
// However, editor specific config, which the server doesn't know about, should
// be specified directly in `package.json`.
//
// To deprecate an option by replacing it with another name use `new_name | `old_name` so that we keep
// parsing the old name.
server_config_data! {
    struct ServerConfigData {
        /// How imports should be grouped into use statements.
        assist_importGranularity |
        assist_importMergeBehavior |
        assist_importMergeBehaviour: ImportGranularityDef  = "\"crate\"",
        /// Whether to enforce the import granularity setting for all files. If set to false husky-analyzer will try to keep import styles consistent per file.
        assist_importEnforceGranularity: bool              = "false",
        /// The path structure for newly inserted paths to use.
        assist_importPrefix: ImportPrefixDef               = "\"plain\"",
        /// Group inserted imports by the https://husky-analyzer.github.io/manual.html#auto-import[following order]. Groups are separated by newlines.
        assist_importGroup: bool                           = "true",
        /// Whether to allow import insertion to merge new imports into single path glob imports like `use std::fmt::*;`.
        assist_allowMergingIntoGlobImports: bool           = "true",

        /// Warm up caches on project load.
        enable_cache_prefill: bool = "true",

        /// Show function name and docs in parameter hints.
        callInfo_full: bool                                = "true",

        /// Automatically refresh project info via `cargo metadata` on
        /// `Cargo.toml` changes.
        cargo_autoreload: bool           = "true",
        /// Activate all available features (`--all-features`).
        cargo_allFeatures: bool          = "false",
        /// Unsets `#[cfg(test)]` for the specified crates.
        cargo_unsetTest: Vec<String>   = "[\"core\"]",
        /// List of features to activate.
        cargo_features: Vec<String>      = "[]",
        /// Run build scripts (`build.rs`) for more precise code analysis.
        cargo_runBuildScripts |
        cargo_loadOutDirsFromCheck: bool = "true",
        /// Use `HUSKYC_WRAPPER=husky-analyzer` when running build scripts to
        /// avoid compiling unnecessary things.
        cargo_useHuskycWrapperForBuildScripts: bool = "true",
        /// Do not activate the `default` feature.
        cargo_noDefaultFeatures: bool    = "false",
        /// Compilation target (target triple).
        cargo_target: Option<String>     = "null",
        /// Internal config for debugging, disables loading of sysroot crates.
        cargo_noSysroot: bool            = "false",

        /// Run specified `cargo check` command for diagnostics on save.
        checkOnSave_enable: bool                         = "true",
        /// Check with all features (`--all-features`).
        /// Defaults to `#husky-analyzer.cargo.allFeatures#`.
        checkOnSave_allFeatures: Option<bool>            = "null",
        /// Check all targets and tests (`--all-targets`).
        checkOnSave_allTargets: bool                     = "true",
        /// Cargo command to use for `cargo check`.
        checkOnSave_command: String                      = "\"check\"",
        /// Do not activate the `default` feature.
        checkOnSave_noDefaultFeatures: Option<bool>      = "null",
        /// Check for a specific target. Defaults to
        /// `#husky-analyzer.cargo.target#`.
        checkOnSave_target: Option<String>               = "null",
        /// Extra arguments for `cargo check`.
        checkOnSave_extraArgs: Vec<String>               = "[]",
        /// List of features to activate. Defaults to
        /// `#husky-analyzer.cargo.features#`.
        checkOnSave_features: Option<Vec<String>>        = "null",
        /// Advanced option, fully override the command husky-analyzer uses for
        /// checking. The command should include `--message-format=json` or
        /// similar option.
        checkOnSave_overrideCommand: Option<Vec<String>> = "null",

        /// Whether to add argument snippets when completing functions.
        /// Only applies when `#husky-analyzer.completion.addCallParenthesis#` is set.
        completion_addCallArgumentSnippets: bool = "true",
        /// Whether to add parenthesis when completing functions.
        completion_addCallParenthesis: bool      = "true",
        /// Custom completion snippets.
        completion_snippets: FxHashMap<String, SnippetDef> = "{}",
        /// Whether to show postfix snippets like `dbg`, `if`, `not`, etc.
        completion_postfix_enable: bool          = "true",
        /// Toggles the additional completions that automatically add imports when completed.
        /// Note that your client must specify the `additionalTextEdits` LSP client capability to truly have this feature enabled.
        completion_autoimport_enable: bool       = "true",
        /// Toggles the additional completions that automatically show method calls and field accesses
        /// with `self` prefixed to them when inside a method.
        completion_autoself_enable: bool       = "true",

        /// Whether to show native husky-analyzer diagnostics.
        enable_diagnostics: bool                = "true",
        /// List of husky-analyzer diagnostics to disable.
        diagnostics_disabled: FxHashSet<String> = "[]",
        /// Map of prefixes to be substituted when parsing diagnostic file paths.
        /// This should be the reverse mapping of what is passed to `huskyc` as `--remap-path-prefix`.
        diagnostics_remapPrefix: FxHashMap<String, String> = "{}",
        /// List of warnings that should be displayed with hint severity.
        ///
        /// The warnings will be indicated by faded text or three dots in code
        /// and will not show up in the `Problems Panel`.
        diagnostics_warningsAsHint: Vec<String> = "[]",
        /// List of warnings that should be displayed with info severity.
        ///
        /// The warnings will be indicated by a blue squiggly underline in code
        /// and a blue icon in the `Problems Panel`.
        diagnostics_warningsAsInfo: Vec<String> = "[]",

        /// Expand attribute macros.
        experimental_procAttrMacros: bool = "true",

        /// Controls file watching implementation.
        files_watcher: String = "\"client\"",
        /// These directories will be ignored by husky-analyzer. They are
        /// relative to the workspace root, and globs are not supported. You may
        /// also need to add the folders to Code's `files.watcherExclude`.
        files_excludeDirs: Vec<PathBuf> = "[]",

        /// Enables highlighting of related references while hovering your mouse above any identifier.
        highlightRelated_references: bool = "true",
        /// Enables highlighting of all exit points while hovering your mouse above any `return`, `?`, or return type arrow (`->`).
        highlightRelated_exitPoints: bool = "true",
        /// Enables highlighting of related references while hovering your mouse `break`, `loop`, `while`, or `for` keywords.
        highlightRelated_breakPoints: bool = "true",
        /// Enables highlighting of all break points for a loop or block context while hovering your mouse above any `async` or `await` keywords.
        highlightRelated_yieldPoints: bool = "true",

        /// Use semantic tokens for strings.
        ///
        /// In some editors (e.g. vscode) semantic tokens override other highlighting grammars.
        /// By disabling semantic tokens for strings, other grammars can be used to highlight
        /// their contents.
        highlighting_strings: bool = "true",

        /// Whether to show documentation on hover.
        hover_documentation: bool       = "true",
        /// Use markdown syntax for links in hover.
        hover_linksInHover |
        hoverActions_linksInHover: bool = "true",

        /// Whether to show `Debug` action. Only applies when
        /// `#husky-analyzer.hoverActions.enable#` is set.
        hoverActions_debug: bool           = "true",
        /// Whether to show HoverActions in Husky files.
        hoverActions_enable: bool          = "true",
        /// Whether to show `Go to Type Definition` action. Only applies when
        /// `#husky-analyzer.hoverActions.enable#` is set.
        hoverActions_gotoTypeDef: bool     = "true",
        /// Whether to show `Implementations` action. Only applies when
        /// `#husky-analyzer.hoverActions.enable#` is set.
        hoverActions_implementations: bool = "true",
        /// Whether to show `References` action. Only applies when
        /// `#husky-analyzer.hoverActions.enable#` is set.
        hoverActions_references: bool      = "false",
        /// Whether to show `Run` action. Only applies when
        /// `#husky-analyzer.hoverActions.enable#` is set.
        hoverActions_run: bool             = "true",

        /// Whether to show inlay type hints for method chains.
        inlayHints_chainingHints: bool              = "true",
        /// Maximum length for inlay hints. Set to null to have an unlimited length.
        inlayHints_maxLength: Option<usize>         = "25",
        /// Whether to show function parameter name inlay hints at the call
        /// site.
        inlayHints_parameterHints: bool             = "true",
        /// Whether to show inlay type hints for variables.
        inlayHints_typeHints: bool                  = "true",
        /// Whether to hide inlay hints for constructors.
        inlayHints_hideNamedConstructorHints: bool  = "false",

        /// Join lines inserts else between consecutive ifs.
        joinLines_joinElseIf: bool = "true",
        /// Join lines removes trailing commas.
        joinLines_removeTrailingComma: bool = "true",
        /// Join lines unwraps trivial blocks.
        joinLines_unwrapTrivialBlock: bool = "true",
        /// Join lines merges consecutive declaration and initialization of an assignment.
        joinLines_joinAssignments: bool = "true",

        /// Whether to show `Debug` lens. Only applies when
        /// `#husky-analyzer.lens.enable#` is set.
        lens_debug: bool            = "true",
        /// Whether to show CodeLens in Husky files.
        lens_enable: bool           = "true",
        /// Whether to show `Implementations` lens. Only applies when
        /// `#husky-analyzer.lens.enable#` is set.
        lens_implementations: bool  = "true",
        /// Whether to show `Run` lens. Only applies when
        /// `#husky-analyzer.lens.enable#` is set.
        lens_run: bool              = "true",
        /// Whether to show `Method References` lens. Only applies when
        /// `#husky-analyzer.lens.enable#` is set.
        lens_methodReferences: bool = "false",
        /// Whether to show `References` lens for Struct, Enum, Union and Trait.
        /// Only applies when `#husky-analyzer.lens.enable#` is set.
        lens_references: bool = "false",
        /// Whether to show `References` lens for Enum Variants.
        /// Only applies when `#husky-analyzer.lens.enable#` is set.
        lens_enumVariantReferences: bool = "false",
        /// Internal config: use custom client-side commands even when the
        /// client doesn't set the corresponding capability.
        lens_forceCustomCommands: bool = "true",

        /// Number of syntax trees husky-analyzer keeps in memory. Defaults to 128.
        lru_capacity: Option<usize>                 = "null",

        /// Whether to show `can't find Cargo.toml` error message.
        notifications_cargoTomlNotFound: bool      = "true",

        /// Enable support for procedural macros, implies `#husky-analyzer.cargo.runBuildScripts#`.
        procMacro_enable: bool                     = "true",
        /// Internal config, path to proc-macro server executable (typically,
        /// this is husky-analyzer itself, but we override this in tests).
        procMacro_server: Option<PathBuf>          = "null",

        /// Command to be executed instead of 'cargo' for runnables.
        runnables_overrideCargo: Option<String> = "null",
        /// Additional arguments to be passed to cargo for runnables such as
        /// tests or binaries. For example, it may be `--release`.
        runnables_cargoExtraArgs: Vec<String>   = "[]",

        /// Path to the Cargo.toml of the husky compiler workspace, for usage in huskyc_private
        /// projects, or "discover" to try to automatically find it if the `huskyc-dev` component
        /// is installed.
        ///
        /// Any project which uses husky-analyzer with the huskycPrivate
        /// crates must set `[package.metadata.husky-analyzer] huskyc_private=true` to use it.
        ///
        /// This option does not take effect until husky-analyzer is restarted.
        huskycSource: Option<String> = "null",

        /// Additional arguments to `huskyfmt`.
        huskyfmt_extraArgs: Vec<String>               = "[]",
        /// Advanced option, fully override the command husky-analyzer uses for
        /// formatting.
        huskyfmt_overrideCommand: Option<Vec<String>> = "null",
        /// Enables the use of huskyfmt's unstable range formatting command for the
        /// `textDocument/rangeFormatting` request. The huskyfmt option is unstable and only
        /// available on a nightly build.
        huskyfmt_enableRangeFormatting: bool = "false",
    }
}

impl Default for ServerConfigData {
    fn default() -> Self {
        ServerConfigData::from_json(serde_json::Value::Null)
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    pub(crate) client_capabilities: lsp_types::ClientCapabilities,
    pub(crate) root_path: AbsPathBuf,
    data: ServerConfigData,
    snippets: Vec<Snippet>,
}

impl ServerConfig {
    pub fn detached_files(&self) -> &[AbsPathBuf] {
        todo!()
    }
    fn update(&mut self, mut _json: serde_json::Value) {
        todo!()
    }

    pub fn new(init_params: lsp_types::InitializeParams) -> Result<ServerConfig> {
        trace_client_info(init_params.client_info);

        let root_path = get_root_path_from_initialize_params(init_params.root_uri)?;

        let mut config = ServerConfig {
            client_capabilities: init_params.capabilities,
            root_path,
            data: ServerConfigData::default(),
            snippets: Default::default(),
        };

        if let Some(json) = init_params.initialization_options {
            config.update(json);
        }

        Ok(config)
    }

    pub fn lru_capacity(&self) -> Option<usize> {
        self.data.lru_capacity
    }
    pub fn enable_cache_prefill(&self) -> bool {
        self.data.enable_cache_prefill
    }
    pub fn enable_diagnostics(&self) -> bool {
        self.data.enable_diagnostics
    }
    pub fn enable_semantic_tokens_refresh(&self) -> bool {
        try_or!(
            self.client_capabilities
                .workspace
                .as_ref()?
                .semantic_tokens
                .as_ref()?
                .refresh_support?,
            false
        )
    }
    pub fn enable_insert_replace(&self) -> bool {
        try_or!(
            self.client_capabilities
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .insert_replace_support?,
            false
        )
    }
    pub fn enable_hierarchical_symbols(&self) -> bool {
        try_or!(
            self.client_capabilities
                .text_document
                .as_ref()?
                .document_symbol
                .as_ref()?
                .hierarchical_document_symbol_support?,
            false
        )
    }
    pub fn line_folding_only(&self) -> bool {
        try_or!(
            self.client_capabilities
                .text_document
                .as_ref()?
                .folding_range
                .as_ref()?
                .line_folding_only?,
            false
        )
    }
    pub fn offset_encoding(&self) -> OffsetEncoding {
        if lsp_ext::supports_utf8(&self.client_capabilities) {
            OffsetEncoding::Utf8
        } else {
            OffsetEncoding::Utf16
        }
    }

    pub fn completion(&self) -> CompletionConfig {
        CompletionConfig {
            enable_postfix_completions: self.data.completion_postfix_enable,
            enable_imports_on_the_fly: self.data.completion_autoimport_enable
                && completion_item_edit_resolve(&self.client_capabilities),
            enable_self_on_the_fly: self.data.completion_autoself_enable,
            add_call_parenthesis: self.data.completion_addCallParenthesis,
            add_call_argument_snippets: self.data.completion_addCallArgumentSnippets,
            insert_use: self.insert_use_config(),
            snippet_cap: SnippetCap::new(try_or!(
                self.client_capabilities
                    .text_document
                    .as_ref()?
                    .completion
                    .as_ref()?
                    .completion_item
                    .as_ref()?
                    .snippet_support?,
                false
            )),
            snippets: self.snippets.clone(),
        }
    }

    pub fn fmt(&self) -> huskyfmt::HuskyfmtConfig {
        match &self.data.huskyfmt_overrideCommand {
            Some(args) if !args.is_empty() => {
                let mut args = args.clone();
                let command = args.remove(0);
                huskyfmt::HuskyfmtConfig::CustomCommand { command, args }
            }
            Some(_) | None => huskyfmt::HuskyfmtConfig::Huskyfmt {
                extra_args: self.data.huskyfmt_extraArgs.clone(),
                enable_range_formatting: self.data.huskyfmt_enableRangeFormatting,
            },
        }
    }

    fn insert_use_config(&self) -> InsertUseConfig {
        todo!()
        // InsertUseConfig {
        //     granularity: match self.data.assist_importGranularity {
        //         ImportGranularityDef::Preserve => ImportGranularity::Preserve,
        //         ImportGranularityDef::Item => ImportGranularity::Item,
        //         ImportGranularityDef::Crate => ImportGranularity::Crate,
        //         ImportGranularityDef::Module => ImportGranularity::Module,
        //     },
        //     enforce_granularity: self.data.assist_importEnforceGranularity,
        //     prefix_kind: todo!(),
        //     // match self.data.assist_importPrefix {
        //     //     ImportPrefixDef::Plain => PrefixKind::Plain,
        //     //     ImportPrefixDef::ByCrate => PrefixKind::ByCrate,
        //     //     ImportPrefixDef::BySelf => PrefixKind::BySelf,
        //     // },
        // }
    }

    pub fn highlighting_strings(&self) -> bool {
        self.data.highlighting_strings
    }

    pub fn highlight_related(&self) -> HighlightRelatedConfig {
        HighlightRelatedConfig {
            references: self.data.highlightRelated_references,
            break_points: self.data.highlightRelated_breakPoints,
            exit_points: self.data.highlightRelated_exitPoints,
            yield_points: self.data.highlightRelated_yieldPoints,
        }
    }
}

fn trace_client_info(client_info: Option<lsp_types::ClientInfo>) {
    if let Some(client_info) = client_info {
        tracing::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.unwrap_or_default()
        );
    }
}

fn get_root_path_from_initialize_params(root_uri: Option<lsp_types::Url>) -> Result<AbsPathBuf> {
    Ok(
        match root_uri
            .and_then(|it| it.to_file_path().ok())
            .and_then(|it| AbsPathBuf::try_from(it).ok())
        {
            Some(it) => it,
            None => {
                let _cwd = std::env::current_dir()?;
                todo!()
                // AbsPathBuf::assert(cwd)
            }
        },
    )
}

fn get_field<T: DeserializeOwned>(
    json: &mut serde_json::Value,
    field: &'static str,
    alias: Option<&'static str>,
    default: &str,
) -> T {
    let default = serde_json::from_str(default).unwrap();

    // XXX: check alias first, to work-around the VS Code where it pre-fills the
    // defaults instead of sending an empty object.
    alias
        .into_iter()
        .chain(core::iter::once(field))
        .find_map(move |field| {
            let mut pointer = field.replace('_', "/");
            pointer.insert(0, '/');
            json.pointer_mut(&pointer)
                .and_then(|it| serde_json::from_value(it.take()).ok())
        })
        .unwrap_or(default)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ImportGranularityDef {
    Preserve,
    #[serde(alias = "none")]
    Item,
    #[serde(alias = "full")]
    Crate,
    #[serde(alias = "last")]
    Module,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
enum ImportPrefixDef {
    Plain,
    #[serde(alias = "self")]
    BySelf,
    #[serde(alias = "crate")]
    ByCrate,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum SnippetScopeDef {
    Expr,
    Item,
    Type,
}

impl Default for SnippetScopeDef {
    fn default() -> Self {
        SnippetScopeDef::Expr
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
struct SnippetDef {
    #[serde(deserialize_with = "single_or_array")]
    prefix: Vec<String>,
    #[serde(deserialize_with = "single_or_array")]
    postfix: Vec<String>,
    description: Option<String>,
    #[serde(deserialize_with = "single_or_array")]
    body: Vec<String>,
    #[serde(deserialize_with = "single_or_array")]
    requires: Vec<String>,
    scope: SnippetScopeDef,
}

fn single_or_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct SingleOrVec;

    impl<'de> serde::de::Visitor<'de> for SingleOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or array of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Deserialize::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(SingleOrVec)
}
