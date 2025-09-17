use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_rating(rating: &BigDecimal) -> Result<(), ValidationError> {
    if *rating < BigDecimal::from(0) || *rating > BigDecimal::from(100) {
        return Err(ValidationError::new("rating_out_of_range"));
    }
    Ok(())
}
