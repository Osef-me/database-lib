use crate::define_insert_returning_id;
use crate::models::beatmaps::beatmapset::types::BeatmapsetRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "beatmapset",
    BeatmapsetRow,
    osu_id,
    artist,
    artist_unicode,
    title,
    title_unicode,
    creator,
    source,
    tags,
    has_video,
    has_storyboard,
    is_explicit,
    is_featured,
    cover_url,
    preview_url,
    osu_file_url
);
