use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

use crate::models::msd::validators::*;

#[derive(Debug, Clone, Validate)]
pub struct MSDRow {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: Option<i32>,

    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: Option<i32>,

    #[validate(custom(function = "validate_msd_value"))]
    pub overall: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub stream: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub jumpstream: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub handstream: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub stamina: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub jackspeed: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub chordjack: Option<BigDecimal>,

    #[validate(custom(function = "validate_msd_value"))]
    pub technical: Option<BigDecimal>,

    #[validate(custom(function = "validate_rate_value"))]
    pub rate: Option<BigDecimal>,

    #[validate(custom(function = "validate_main_pattern"))]
    pub main_pattern: Option<String>,

    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
