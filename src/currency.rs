use std::{fmt::Display, str::FromStr};
use eyre::eyre;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error("Invalid currency")]
    InvalidCurrency,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl TryFrom<Currency> for iso_currency::Currency {
    type Error = eyre::Report;

    fn try_from(value: Currency) -> Result<Self, Self::Error> {
        iso_currency::Currency::from_code(value.code()).ok_or(eyre!("Invalid currency"))
    }
}

impl TryFrom<iso_currency::Currency> for Currency {
    type Error = eyre::Report;

    fn try_from(value: iso_currency::Currency) -> Result<Self, Self::Error> {
        Currency::from_code(value.code()).ok_or(eyre!("Invalid currency"))
    }
}


impl Currency {
    // ISO 4217 Codes
    pub fn code(&self) -> &str {
        match self {
            Currency::EUR => "EUR",
            Currency::USD => "USD",
        }
    }

    pub fn from_code(code: &str) -> Option<Currency> {
        match code {
            "EUR" => Some(Self::EUR),
            "USD" => Some(Self::USD),
            _ => None,
        }
    }

    pub fn from_iso_4127_number(code: &str) -> Option<Currency> {
        match code {
            "978" => Some(Self::EUR),
            "840" => Some(Self::USD),
            _ => None,
        }
    }

    pub fn iso_4127_num(&self) -> &str {
        match self {
            Self::EUR => "978",
            Self::USD => "840",
        }
    }

    pub fn minor_unit_scale(&self) -> u32 {
        match self {
            Self::EUR => 2,
            Self::USD => 2,
        }
    }

    // Precision level
    pub fn max_scale(&self) -> u32 {
        10
    }

    pub fn to_prefix(&self) -> &str {
        match self {
            Self::EUR => "€",
            Self::USD => "$",
        }
    }

}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Currency {
    type Err = CurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s).ok_or(CurrencyError::InvalidCurrency)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code_serialize() {
        assert_eq!(
            serde_json::to_string(&Currency::EUR).expect("unexpected error"),
            r#""EUR""#
        );
    }

    #[test]
    fn test_currency_code_deserialize() {
        assert_eq!(
            serde_json::from_str::<Currency>(r#""EUR""#).expect("unexpected error"),
            Currency::EUR
        );
    }
}