use ecow::{eco_format, EcoString};

use crate::diag::{At, SourceResult};
use crate::engine::TexEngine;
use crate::foundations::{func, scope, Str, TexValue};
use crate::loading::Readable;
use crate::syntax::Spanned;
use crate::IsTexWorld;

/// Reads structured data from a YAML file.
///
/// The file must contain a valid YAML object or array. YAML mappings will be
/// converted into Tex dictionaries, and YAML sequences will be converted into
/// Tex arrays. Strings and booleans will be converted into the Tex
/// equivalents, null-values (`null`, `~` or empty ``) will be converted into
/// `{none}`, and numbers will be converted to floats or integers depending on
/// whether they are whole numbers. Custom YAML tags are ignored, though the
/// loaded value will still be present.
///
/// The YAML files in the example contain objects with authors as keys,
/// each with a sequence of their own submapping with the keys
/// "title" and "published"
///
/// # Example
/// ```example
/// #let bookshelf(contents) = {
///   for (author, works) in contents {
///     author
///     for work in works [
///       - #work.title (#work.published)
///     ]
///   }
/// }
///
/// #bookshelf(
///   yaml("scifi-authors.yaml")
/// )
/// ```
#[func(scope, title = "YAML")]
pub fn yaml(
    /// The engine.
    engine: &mut TexEngine,
    /// Path to a YAML file.
    path: Spanned<EcoString>,
) -> SourceResult<TexValue> {
    let Spanned { v: path, span } = path;
    let id = span.resolve_path(&path).at(span)?;
    let data = engine.world.file(id).at(span)?;
    yaml::decode(Spanned::new(Readable::Bytes(data), span))
}

#[scope]
impl yaml {
    /// Reads structured data from a YAML string/bytes.
    #[func(title = "Decode YAML")]
    pub fn decode(
        /// YAML data.
        data: Spanned<Readable>,
    ) -> SourceResult<TexValue> {
        let Spanned { v: data, span } = data;
        serde_yaml::from_slice(data.as_slice())
            .map_err(|err| eco_format!("failed to parse YAML ({err})"))
            .at(span)
    }

    /// Encode structured data into a YAML string.
    #[func(title = "Encode YAML")]
    pub fn encode(
        /// TexValue to be encoded.
        value: Spanned<TexValue>,
    ) -> SourceResult<Str> {
        let Spanned { v: value, span } = value;
        serde_yaml::to_string(&value)
            .map(|v| v.into())
            .map_err(|err| eco_format!("failed to encode value as YAML ({err})"))
            .at(span)
    }
}
