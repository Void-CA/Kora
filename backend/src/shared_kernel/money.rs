use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Currency {
    USD,
    NIO,
}

#[derive(Debug, Clone, Copy)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn add(&self, other: &Money) -> Result<Self, RateError> {
        if self.currency != other.currency {
            return Err(RateError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Money::new(self.amount + other.amount, self.currency))
    }

    pub fn subtract(&self, other: &Money) -> Result<Self, RateError> {
        if self.currency != other.currency {
            return Err(RateError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Money::new(self.amount - other.amount, self.currency))
    }

    pub fn convert_to(
        &self,
        target: Currency,
        provider: &dyn ExchangeRateProvider,
    ) -> Result<Self, RateError> {
        if self.currency == target {
            return Ok(self.clone());
        }
        let rate = provider.get_rate(self.currency, target)?;
        let new_amount = self.amount * rate;
        Ok(Money::new(new_amount, target))
    }
}

pub trait ExchangeRateProvider {
    fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, RateError>;
}

#[derive(Debug, Error, PartialEq)]
pub enum RateError {
    #[error("Currency mismatch: cannot convert {0:?} to {1:?}")]
    CurrencyMismatch(Currency, Currency),
    #[error("Rate not available for {0:?} to {1:?}")]
    RateNotAvailable(Currency, Currency),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    struct MockRateProvider;
    impl ExchangeRateProvider for MockRateProvider {
        fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, RateError> {
            match (from, to) {
                (Currency::USD, Currency::NIO) => Ok(Decimal::from(36)),
                (Currency::NIO, Currency::USD) => Ok(Decimal::from_str("0.0277").unwrap()),
                _ => Err(RateError::RateNotAvailable(from, to)),
            }
        }
    }

    #[test]
    fn money_new_and_fields() {
        let m = Money::new(Decimal::from(100), Currency::USD);
        assert_eq!(m.amount, Decimal::from(100));
        assert_eq!(m.currency, Currency::USD);
    }

    #[test]
    fn money_add_same_currency() {
        let m1 = Money::new(Decimal::from(100), Currency::USD);
        let m2 = Money::new(Decimal::from(50), Currency::USD);
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.amount, Decimal::from(150));
    }

    #[test]
    fn money_subtract_same_currency() {
        let m1 = Money::new(Decimal::from(100), Currency::USD);
        let m2 = Money::new(Decimal::from(30), Currency::USD);
        let result = m1.subtract(&m2).unwrap();
        assert_eq!(result.amount, Decimal::from(70));
    }

    #[test]
    fn money_add_different_currency_error() {
        let m1 = Money::new(Decimal::from(100), Currency::USD);
        let m2 = Money::new(Decimal::from(50), Currency::NIO);
        let result = m1.add(&m2);
        assert!(result.is_err());
    }

    #[test]
    fn money_convert_usd_to_nio() {
        let m = Money::new(Decimal::from(100), Currency::USD);
        let provider = MockRateProvider;
        let result = m.convert_to(Currency::NIO, &provider).unwrap();
        assert_eq!(result.amount, Decimal::from(3600));
        assert_eq!(result.currency, Currency::NIO);
    }
}
