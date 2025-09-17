use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex pour valider les hash alphanumériques
    pub static ref HASH_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    /// Regex pour valider les emails
    pub static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    /// Regex pour valider les noms d'utilisateur (alphanumériques + underscore)
    pub static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    /// Regex pour valider les UUIDs
    pub static ref UUID_REGEX: Regex = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();

    /// Regex pour valider les types de rating (etterna, osu, quaver, malody)
    pub static ref RATING_TYPE_REGEX: Regex = Regex::new(r"^(etterna|osu|quaver|malody)$").unwrap();

    pub static ref RANK_REGEX: Regex = Regex::new(r"^(SS|S|A|B|C|D|F|FAILED)$").unwrap();

    pub static ref SCORE_STATUS_REGEX: Regex = Regex::new(r"^(pending|processing|validated|cheated|unsubmitted)$").unwrap();
}
