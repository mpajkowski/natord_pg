use pgrx::*;

use std::cmp::Ordering;

pg_module_magic!();

extension_sql_file!(
    "../sql/operators.sql",
    finalize //requires = [natord_gt, natord_lt, natord_ge, natord_le, natord_cmp]
);

#[pg_extern]
fn natord_gt(lhs: &str, rhs: &str) -> bool {
    natord::compare(lhs, rhs) == Ordering::Greater
}

#[pg_extern]
fn natord_lt(lhs: &str, rhs: &str) -> bool {
    natord::compare(lhs, rhs) == Ordering::Less
}

#[pg_extern]
fn natord_ge(lhs: &str, rhs: &str) -> bool {
    matches!(
        natord::compare(lhs, rhs),
        Ordering::Greater | Ordering::Equal
    )
}

#[pg_extern]
fn natord_le(lhs: &str, rhs: &str) -> bool {
    matches!(natord::compare(lhs, rhs), Ordering::Less | Ordering::Equal)
}

#[pg_extern]
fn natord_cmp(lhs: &str, rhs: &str) -> i32 {
    match natord::compare(lhs, rhs) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[test]
    fn test_natord_cmp() {
        assert_eq!(1, natord_cmp("112A", "112"));
        assert_eq!(0, natord_cmp("112A", "112A"));
        assert_eq!(-1, natord_cmp("112A", "112B"));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
