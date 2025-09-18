use crate::define_insert_returning_id;
use crate::models::user::device_tokens::types::DeviceTokensRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "device_tokens",
    DeviceTokensRow,
    token,
    discord_id,
    device_name,
    hwid
);
