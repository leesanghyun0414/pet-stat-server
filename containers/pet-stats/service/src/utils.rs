use chrono::{DateTime, FixedOffset, Local};
use sea_orm::{DbConn, DbErr, TransactionTrait};
use tracing::{error, trace};

pub async fn start_transaction(db: &DbConn) -> Result<sea_orm::DatabaseTransaction, DbErr> {
    match db.begin().await {
        Ok(txn) => {
            trace!("Database transaction started successfully");
            Ok(txn)
        }
        Err(e) => {
            error!("Failed to start database transaction: {:?}", e);
            Err(e)
        }
    }
}

pub async fn commit_transaction(txn: sea_orm::DatabaseTransaction) -> Result<(), DbErr> {
    match txn.commit().await {
        Ok(_) => {
            trace!("Transaction committed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to commit transaction: {:?}", e);
            Err(e)
        }
    }
}

pub(crate) fn get_current_time() -> DateTime<FixedOffset> {
    Local::now().with_timezone(Local::now().offset())
}
