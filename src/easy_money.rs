// This is a file that is supposed to represent a money type in Rust
// This type doesn't need to be precise

use eyre::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

use super::currency::Currency;
use super::money_util::{MoneyUtil, MoneyError, RawMoney};
use super::money::Money;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMoney")]
pub struct EasyMoney {
    #[serde(serialize_with = "rust_decimal::serde::str::serialize")]
    pub amount: Decimal,
    pub currency: Currency,
}

impl EasyMoney {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, MoneyError> {
        let amount = MoneyUtil::check_amount_validation(amount, currency.minor_unit_scale(), currency.minor_unit_scale())?;
        Ok(EasyMoney { amount, currency })
    }

    pub fn from_str_amount(amount: &str, currency: Currency) -> Result<Self, MoneyError> {
        let amount = Decimal::from_str(amount)?;
        Self::new(amount, currency)
    }
}

impl TryFrom<Money> for EasyMoney {
    type Error = MoneyError;

    fn try_from(value: Money) -> Result<Self, Self::Error> {
        EasyMoney::new(value.amount, value.currency)
    }
}

impl TryFrom<RawMoney> for EasyMoney {
    type Error = MoneyError;

    fn try_from(value: RawMoney) -> Result<Self, Self::Error> {
        EasyMoney::new(value.amount, value.currency)
    }
}

#[cfg(test)]
/// This module contains tests for the `EasyMoney` struct in the `easy_money.rs` file.
mod tests {
    use super::*;
    use std::str::FromStr;
    use rstest::rstest;

    /// Test cases for the `EasyMoney` struct.
    #[rstest]
    #[case("1", Currency::EUR, "1.00")]
    #[case("1.230", Currency::EUR, "1.23")]
    // 10.20000 is a good case because when Decimal(10.20000) is converted to flaot, it becomes 10.2000...001 which is 10.20
    #[case("10.20000", Currency::EUR, "10.20")]
    #[case("10.0300", Currency::EUR, "10.03")]
    fn test_easy_money(
        #[case] amount: &str, 
        #[case] currency: Currency, 
        #[case] expected: &str,
    ) {
        let some_money = EasyMoney::from_str_amount(amount, currency).unwrap();
        let expected_decimal_value = Decimal::from_str(expected).unwrap();

        // Validate the simple money's amount in still the same 
        assert_eq!(some_money.amount, expected_decimal_value);
        // Validate the currency is the same
        assert_eq!(some_money.currency, currency);
        // Validate the amount has scale that is expected
        assert_eq!(some_money.amount.to_string() , expected);
    }
}

