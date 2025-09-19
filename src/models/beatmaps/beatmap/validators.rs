use bigdecimal::{BigDecimal, FromPrimitive};
use validator::ValidationError;

pub fn validate_od_hp_cs(value: &BigDecimal) -> Result<(), ValidationError> {
    if value < &BigDecimal::from_f64(0.0).unwrap() || value > &BigDecimal::from_f64(10.0).unwrap() {
        return Err(validator::ValidationError::new("invalid_od_hp_cs"));
    }
    Ok(())
}

pub fn validate_ar(value: &BigDecimal) -> Result<(), ValidationError> {
    if value < &BigDecimal::from_f64(0.0).unwrap() || value > &BigDecimal::from_f64(11.0).unwrap() {
        return Err(validator::ValidationError::new("invalid_ar"));
    }
    Ok(())
}
