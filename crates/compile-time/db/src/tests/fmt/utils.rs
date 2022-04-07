use crate::*;

pub(super) fn test_invariance_under_fmt(original: &'static str) {
    let original = original.trim_matches(|c| c == '\n' || c == ' ');
    let mut db = HuskyLangCompileTime::default();
    db.set_live_file_text("haha/main.hsk".into(), original.into());
    let main_file_id = db.intern_file("haha/main.hsk".into());
    let fmt_text = db.fmt_text(main_file_id).unwrap();
    should_eq!(fmt_text.as_ref(), original);
}
