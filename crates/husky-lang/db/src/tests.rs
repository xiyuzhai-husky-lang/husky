use crate::*;

use diagnostic::DiagnosticQuery;
use file::LiveFiles;

use test_data::*;

#[test]
fn test_diagnostics() {
    let mut db = HuskyLangDatabase::default();
    db.set_live_file_text(play_main_path(), play_main_text());
    let modules = db.all_modules();
    assert_eq!(modules.len(), 1);
    let the_module = modules[0];
    let diagnostic_reserve0 = db.diagnostic_reserve(the_module);
    ep!(diagnostic_reserve0);
    assert_eq!(diagnostic_reserve0.diagnostics_ref().len(), 0);
}
