use async_graphql::{Error, ErrorExtensions};
use sea_orm::DbErr;

#[inline]
fn gql_err(code: &'static str, msg: impl Into<String>) -> Error {
    Error::new(msg).extend_with(|_, e| e.set("code", code))
}

pub fn db_err_to_gql(err: DbErr) -> Error {
    match err {
        DbErr::Custom(s) if s == "TOKEN_EXPIRED" => {
            gql_err("TOKEN_EXPIRED", "Expired refresh token")
        }
        other => gql_err("OTHER_ERROR", other.to_string()),
    }
}
