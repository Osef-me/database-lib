use crate::define_insert_returning_id;
use crate::models::users::new_users::types::NewUsersRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "new_users",
    NewUsersRow,
    discord_id,
    username,
    token
);
