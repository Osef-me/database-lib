use crate::define_insert_returning_id;
use crate::models::scores::replay::types::ReplayRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "replay",
    ReplayRow,
    replay_hash,
    replay_available,
    replay_path
);
