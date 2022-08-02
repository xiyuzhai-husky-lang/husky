use std::path::PathBuf;

use husky_debugger::*;
use husky_print_utils::p;
use husky_root_static_defn::{__Linkage, __StaticLinkageKey};
use libloading::{Library, Symbol};

#[tokio::main]
async fn main() {
    let flags = HuskyDebuggerFlags::from_env().expect("invalid arguments");
    let opt_library: Option<Library> = if let Some(cdylib) = flags.cdylib {
        Some(unsafe { Library::new(cdylib) }.expect("it should work"))
    } else {
        None
    };
    let linkages_from_cdylib: &'static [(__StaticLinkageKey, __Linkage)] = opt_library
        .as_ref()
        .map(|library| unsafe {
            library
                .get::<GetLinkagesFromCDylib>(b"get_linkages")
                .expect("what")()
        })
        .unwrap_or(&[]);
    let mode: Mode = flags.mode.into();
    let package_dir: PathBuf = flags.package_dir.unwrap().into();
    mode.apply(&package_dir, linkages_from_cdylib).await
}
