use sea_orm::{
    prelude::Expr,
    sea_query::{ColumnDef, IntoIden},
};

pub(crate) fn current_timestamp_col<T>(col_name: T) -> ColumnDef
where
    T: IntoIden,
{
    ColumnDef::new(col_name)
        .timestamp_with_time_zone()
        .not_null()
        .default(Expr::current_timestamp())
        .to_owned()
}
