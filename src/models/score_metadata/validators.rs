use bigdecimal::BigDecimal;
use validator::ValidationError;

/// Validates that accuracy is within valid range (0-100).
pub fn validate_accuracy(accuracy: &BigDecimal) -> Result<(), ValidationError> {
    if *accuracy < BigDecimal::from(0) || *accuracy > BigDecimal::from(100) {
        return Err(ValidationError::new("accuracy_out_of_range"));
    }
    Ok(())
}
