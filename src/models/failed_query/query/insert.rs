use crate::define_insert_returning_id;
use crate::models::failed_query::types::FailedQueryRow;
// no extra imports needed

define_insert_returning_id!(insert, "failed_query", FailedQueryRow, hash);
