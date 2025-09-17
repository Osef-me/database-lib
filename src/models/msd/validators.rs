use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use validator::ValidationError;

/// Validates that a MSD value is within valid range (0-20) if provided.
pub fn validate_msd_value(value: &BigDecimal) -> Result<(), ValidationError> {
    if *value < BigDecimal::from(0) || *value > BigDecimal::from(100) {
        return Err(ValidationError::new("msd_value_out_of_range"));
    }
    Ok(())
}

/// Validates that a rate value is within valid range (0.5-2.0) if provided.
pub fn validate_rate_value(rate: &BigDecimal) -> Result<(), ValidationError> {
    if *rate < BigDecimal::from_f64(0.5).unwrap() || *rate > BigDecimal::from_f64(2.0).unwrap() {
        return Err(ValidationError::new("rate_value_out_of_range"));
    }
    Ok(())
}

/// Validates that a main pattern is a valid JSON string if provided.
pub fn validate_main_pattern(pattern: &str) -> Result<(), ValidationError> {
    if pattern.is_empty() {
        return Err(ValidationError::new("main_pattern_cannot_be_empty"));
    }
    if pattern.len() > 1000 {
        return Err(ValidationError::new("main_pattern_too_long"));
    }

    // Validate that it's valid JSON
    if serde_json::from_str::<serde_json::Value>(pattern).is_err() {
        return Err(ValidationError::new("main_pattern_invalid_json"));
    }

    Ok(())
}
