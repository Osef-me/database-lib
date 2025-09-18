use crate::define_insert_returning_id;
use crate::models::msd::types::MSDRow;

define_insert_returning_id!(
    insert,
    "msd",
    MSDRow,
    beatmap_id,
    overall,
    stream,
    jumpstream,
    handstream,
    stamina,
    jackspeed,
    chordjack,
    technical,
    rate,
    main_pattern
);
