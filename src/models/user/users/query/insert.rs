use crate::define_insert_returning_id;
use crate::models::user::users::types::UsersRow;
// no extra imports needed

define_insert_returning_id!(insert, "users", UsersRow, discord_id, username, roles);
