use once_cell::sync::Lazy;
use regex::Regex;

/// Regex for Hash alphanumerical
pub static HASH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());
/// Regex for Email
pub static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

/// Regex for Username (alphanumerical + underscore)
pub static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

/// Regex for UUIDs
pub static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap()
});

/// Regex for Rating types (etterna, osu, quaver, malody)
pub static RATING_TYPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(etterna|osu|quaver|malody)$").unwrap());

pub static RANK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(SS|S|A|B|C|D|F|FAILED)$").unwrap());

pub static SCORE_STATUS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(pending|processing|validated|cheated|unsubmitted)$").unwrap());

/// Regex for Beatmap status (pending, ranked, qualified, loved, graveyard)
pub static BEATMAP_STATUS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(pending|ranked|qualified|loved|graveyard)$").unwrap());