use eyre::Result;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::currency::Currency;
use super::money_util::{MoneyError, RawMoney, MoneyUtil};
use crate::EasyMoney;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMoney")]
pub struct Money {
    #[serde(serialize_with = "rust_decimal::serde::str::serialize")]
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, MoneyError> {
        let amount = MoneyUtil::check_amount_validation(amount, currency.minor_unit_scale(), currency.max_scale())?;
        Ok(Money { amount, currency })
    }

    pub fn from_str_amount(amount: &str, currency: Currency) -> Result<Self, MoneyError> {
        let amount = Decimal::from_str(amount)?;
        Self::new(amount, currency)
    }

    pub fn from_iso_4127_number(amount: Decimal, iso_4127_number: &str) -> Result<Self, MoneyError> {
        if let Some(currency) = Currency::from_iso_4127_number(iso_4127_number) {
            Self::new(Decimal::from_str(&amount.to_string())?, currency)
        } else {
            Err(MoneyError::InvalidCurrencyError(format!(
                "Given iso_4127_number {} is invalid.",
                iso_4127_number
            )))
        }
    }

    pub fn to_simple_money(&self) -> Result<EasyMoney, MoneyError> {
        EasyMoney::new(self.amount, self.currency)
    }

    pub fn absolute(&self) -> Self {
        Money {
            amount: self.amount.abs(),
            currency: self.currency,
        }
    }

    pub fn inverse(&self) -> Self {
        Money {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

impl TryFrom<RawMoney> for Money {
    type Error = MoneyError;

    fn try_from(value: RawMoney) -> Result<Self, Self::Error> {
        Money::new(value.amount, value.currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use rstest::rstest;

    #[rstest]
    #[case("1.23000", Currency::EUR, "1.23")]
    #[case("1.23", Currency::EUR, "1.23")]
    #[case("1.230010", Currency::EUR, "1.23001")]
    #[case("1.1234567", Currency::EUR, "1.1234567")]
    // 10.2 cannot be represented as float, it becomes 10.2000000000...01
    // In binary format 10.2 translates into 1.01000010 * 2^3 Which means:
    // 10.2 is represented as 0(sign bit) 1000(exponent or in this case scale) 01000010(binary of 1.01000010 ) so 10.2 in the case of scale =8 will have hanging bit which makes it 10.200...01
    #[case("10.20000", Currency::EUR, "10.20")]
    #[case("-10.20000", Currency::EUR, "-10.20")]
    fn test_money(
        #[case] source_amount: &str, 
        #[case] currency: Currency, 
        #[case] expected: &str,
    ) {
        let some_money = Money::new(Decimal::from_str(source_amount).unwrap(), currency).unwrap();
        let expected_decimal_value = Decimal::from_str(expected).unwrap();

        assert_eq!(some_money.amount, expected_decimal_value);
        assert_eq!(some_money.currency, currency);
        assert_eq!(some_money.amount.to_string() , expected);
    }
}