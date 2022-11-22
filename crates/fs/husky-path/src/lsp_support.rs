use std::path::{self, Path};

fn url_from_abs_path(path: &Path) -> lsp_types::Url {
    let url = lsp_types::Url::from_file_path(path).unwrap();
    match path.components().next() {
        Some(path::Component::Prefix(prefix))
            if matches!(
                prefix.kind(),
                path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
            ) =>
        {
            // Need to lowercase driver letter
        }
        _ => return url,
    }

    let driver_letter_range = {
        use itertools::Itertools;

        let (scheme, drive_letter, _rest) = match url.as_str().splitn(3, ':').collect_tuple() {
            Some(it) => it,
            None => return url,
        };
        let start = scheme.len() + ':'.len_utf8();
        start..(start + drive_letter.len())
    };

    // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // machinery *also* canonicalizes the drive letter. So, just massage the
    // string in place.
    let mut url: String = url.into();
    url[driver_letter_range].make_ascii_lowercase();
    lsp_types::Url::parse(&url).unwrap()
}

// #[cfg(feature = "lsp_support")]
// impl Into<lsp_types::Url> for ModuleSourcePath {
//     fn into(self) -> lsp_types::Url {
//         todo!()
//         // url_from_abs_path(&self)
//     }
// }
