use crate::define_insert_returning_id;
use crate::models::users::bans::types::BansRow;
// no extra imports needed

define_insert_returning_id!(insert, "bans", BansRow, discord_id, reason, banned_at);
