use std::cmp::Ordering;

use pgx::*;

pg_module_magic!();

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
fn natord_cmp(lhs: &str, rhs: &str) -> i32 {
    match natord::compare(lhs, rhs) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[pg_extern]
fn natord_le(lhs: &str, rhs: &str) -> bool {
    matches!(natord::compare(lhs, rhs), Ordering::Less | Ordering::Equal)
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_natord_cmp() {
        assert_eq!(1, natord_cmp("112A", "112"));
        assert_eq!(0, natord_cmp("112A", "112A"));
        assert_eq!(-1, natord_cmp("112A", "112B"));
    }
}
