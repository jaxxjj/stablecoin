use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Below minimum health factor")]
    BelowMinimumHealthFactor,
    #[msg("Health factor above threshold")]
    HealthFactorAboveThreshold,
}
