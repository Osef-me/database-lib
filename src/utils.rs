use once_cell::sync::Lazy;
use regex::Regex;

/// Regex pour valider les hash alphanumériques
pub static HASH_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());
/// Regex pour valider les emails
pub static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

/// Regex pour valider les noms d'utilisateur (alphanumériques + underscore)
pub static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

/// Regex pour valider les UUIDs
pub static UUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap()
});

/// Regex pour valider les types de rating (etterna, osu, quaver, malody)
pub static RATING_TYPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(etterna|osu|quaver|malody)$").unwrap());

pub static RANK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(SS|S|A|B|C|D|F|FAILED)$").unwrap());

pub static SCORE_STATUS_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(pending|processing|validated|cheated|unsubmitted)$").unwrap());
