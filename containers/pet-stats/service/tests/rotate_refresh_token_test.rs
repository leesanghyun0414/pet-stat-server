// use chrono::{Duration, Local};
//
// use sea_orm::{
//     entity::prelude::*, sea_query::Expr, DatabaseBackend, MockDatabase, MockExecResult, Transaction,
// };
//
// use service::auth::refresh_token::RefreshToken;
// use service::auth::{error, refresh_token};
//
// use entity::entities::user_tokens as ut;
// use service::mutations::user::{self as UserQuery, UserMutation};
// use service::queries::user as UserMutation;
//
// const SECRET: [u8; 32] = *b"0123456789ABCDEF0123456789ABCDEF";
//
// fn model(hash: &[u8; 32], uid: i32, revoked: bool) -> ut::Model {
//     ut::ActiveModel {
//         refresh_token: hash.to_vec(),
//         user_id: uid,
//         expires_at: (Local::now() + Duration::days(1)).with_timezone(Local::now().offset()),
//         revoked: revoked,
//         ..Default::default()
//     }
// }
//
// #[tokio::test]
// async fn rotate_success_mock() {
//     let old = RefreshToken::generate().unwrap();
//     let old_hash = old.hash(SECRET.as_slice());
//
//     let new = RefreshToken::generate().unwrap();
//     let new_hash = new.hash(SECRET.as_slice());
//
//     let db = MockDatabase::new(DatabaseBackend::Postgres)
//         .append_exec_results([MockExecResult {
//             last_insert_id: 0,
//             rows_affected: 1,
//         }])
//         .append_query_results([vec![model(&old_hash, 8, false)]])
//         .into_connection();
//
//     let rotated_token =
//         UserMutation::rotate_refresh_token(&db, old_hash.as_slice(), 8, new_hash.as_slice())
//             .await
//             .unwrap();
//
//     assert_eq!(rotated_token.refresh_token, new_hash.to_vec())
// }
