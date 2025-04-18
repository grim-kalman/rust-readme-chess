use rust_readme_chess::utils::common::{column_to_index, row_to_index};

#[test]
fn test_column_to_index() {
    assert_eq!(column_to_index('a'), 0);
    assert_eq!(column_to_index('d'), 3);
    assert_eq!(column_to_index('h'), 7);
}

#[test]
fn test_row_to_index() {
    assert_eq!(row_to_index('8'), 0);
    assert_eq!(row_to_index('5'), 3);
    assert_eq!(row_to_index('1'), 7);
}