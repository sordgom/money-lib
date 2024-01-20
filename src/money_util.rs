use super::currency::Currency;
use eyre::Result;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum MoneyError {
    #[error("InvalidAmountError. {0}")]
    InvalidAmountError(String),
    #[error("Invalid decimal error {0}")]
    DecimalParseError(#[from] rust_decimal::Error),
    #[error("InvalidCurrencyError. {0}")]
    InvalidCurrencyError(String),
    #[error("Internal error. {0}")]
    InternalError(#[from] eyre::Error),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct RawMoney {
    #[serde(serialize_with = "rust_decimal::serde::str::serialize")]
    pub amount: Decimal,
    pub currency: Currency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MoneyUtil {}

impl MoneyUtil {
    // Validating an amount must not exceed the max scale
    pub fn check_amount_validation(
        amount: Decimal,
        min_scale: u32,
        max_scale: u32,
    ) -> Result<Decimal, MoneyError>{
        //if scale = 8, the Decimal will be 0.00000001
        let divisible_by = Decimal::new(1, max_scale);
        let remainder = amount % divisible_by;
        if !remainder.is_zero() {
            return Err(MoneyError::InvalidAmountError(format!(
                "Incorrect scale. Amount {} is not a factor of {}.",
                amount,
                divisible_by
            )));
        }

        let mut amount = amount.normalize();

        // Normalize the number to ensure that it will always match the currency's scale.
        if amount.scale() < min_scale {
            amount.rescale(min_scale);
        }
        Ok(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use rstest::rstest;

    #[rstest]
    #[case("10", 2, 0, "10")]
    #[case("10", 2, 2, "10.00")]
    #[case("10.0", 1, 1, "10.0")]
    #[case("10.000", 2, 2, "10.00")]
    #[case("1.0000012345", 2, 10, "1.0000012345")]
    #[case("1.012345010", 2, 10, "1.01234501")]
    fn test_amount_validation(
        #[case] amount: &str, 
        #[case] min_scale: u32, 
        #[case] max_scale: u32,
        #[case] expected: &str,
    ) {
        let input_decimal = Decimal::from_str(amount).unwrap();
        let output_decimal = MoneyUtil::check_amount_validation(input_decimal, min_scale, max_scale).unwrap();
        let expected_decimal_value = Decimal::from_str(expected).unwrap();
        assert_eq!(output_decimal, expected_decimal_value);
    }

    #[rstest]
    #[case("10.55", 2, 1)]
    #[case("10.12345", 2, 3)]
    fn test_amount_validation_throwing_errors(
        #[case] amount: &str,
        #[case] min_scale: u32,
        #[case] max_scale: u32,
    ) {
        let input_decimal = Decimal::from_str(amount).unwrap();
        let output = MoneyUtil::check_amount_validation(input_decimal, min_scale, max_scale);

        assert!(output.is_err())
    }
}

