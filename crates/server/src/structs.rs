use crate::database::QueriedData;

impl QueriedData for String {
    fn len() -> usize {
        1_usize
    }
    fn create_from_row(row: &tokio_postgres::Row) -> Self {
        row.get(0)
    }
}
