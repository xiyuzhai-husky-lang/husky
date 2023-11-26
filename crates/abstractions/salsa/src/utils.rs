use crate::*;

#[macro_export]
macro_rules! assert_eq_with_db {
    ($db: expr, $left: expr, $right: expr) => {
        if let Err(error_message) = ::salsa::utils::assert_eq_with_db_f($db, &$left, &$right) {
            panic!("{}", error_message)
        }
    };
}

pub fn assert_eq_with_db_f<T>(db: &dyn Database, left: &T, right: &T) -> Result<(), String>
where
    T: PartialEq + DebugWithDb,
{
    if left != right {
        Err(format!(
            r#"assertion failed: `(left == right)`
left: `{:?}`,
right: `{:?}`"#,
            left.debug(db),
            right.debug(db)
        ))
    } else {
        Ok(())
    }
}
