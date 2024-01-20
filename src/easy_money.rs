// This is a file that is supposed to represent a money type in Rust
// This type doesn't need to be precise

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RawMoney")]
pub struct EasyMoney {
    #[serde(serialize_with = "rust_decimal::serde::str::serialize")]
    pub amount: Decimal,
    pub currency: Currency,
}

impl EasyMoney {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, MoneyError> {
        let amount = MoneyUtil::validate_amount(amount, currency.minor_unit_scale(), currency.minor_unit_scale())?;
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