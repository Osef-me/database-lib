use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_rate(rate: &BigDecimal) -> Result<(), ValidationError> {
    if *rate < BigDecimal::from(0) || *rate > BigDecimal::from(10) {
        return Err(ValidationError::new("rate_out_of_range"));
    }
    Ok(())
}
