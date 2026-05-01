// shared_kernel/money.rs
use rust_decimal::Decimal;
use thiserror::Error;


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Currency {
    USD, // Dólares
    NIO, // Córdobas
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

    /// Convierte el monto a otra moneda usando un provider de tasas.
    /// Si la moneda es la misma, retorna self sin cambios.
    pub fn convert_to(
        &self,
        target: Currency,
        provider: &impl ExchangeRateProvider,
    ) -> Result<Self, RateError> {
        if self.currency == target {
            return Ok(self.clone());
        }

        let rate = provider.get_rate(self.currency, target)?;
        // rate = "1 self.currency = X target"
        let new_amount = self.amount * rate;
        Ok(Money::new(new_amount, target))
    }
}

/// Trait para desacoplar la fuente de tasas de cambio.
/// Implementaciones futuras: ManualRateProvider, ApiRateProvider (Banco Central).
pub trait ExchangeRateProvider {
    /// Retorna la tasa: 1 `from` = X `to`
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
    use rust_decimal::Decimal;
    use std::str::FromStr;

    struct MockRateProvider;
    impl ExchangeRateProvider for MockRateProvider {
        fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, RateError> {
            match (from, to) {
                (Currency::USD, Currency::NIO) => Ok(Decimal::from(36)), // 1 USD = 36 NIO
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
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn money_subtract_same_currency() {
        let m1 = Money::new(Decimal::from(100), Currency::USD);
        let m2 = Money::new(Decimal::from(30), Currency::USD);
        let result = m1.subtract(&m2).unwrap();
        assert_eq!(result.amount, Decimal::from(70));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn money_add_different_currency_error() {
        let m1 = Money::new(Decimal::from(100), Currency::USD);
        let m2 = Money::new(Decimal::from(50), Currency::NIO);
        let result = m1.add(&m2);
        assert!(result.is_err());
        match result.unwrap_err() {
            RateError::CurrencyMismatch(..) => (), // OK
            _ => panic!("Expected CurrencyMismatch"),
        }
    }

    #[test]
    fn money_convert_to_same_currency() {
        let m = Money::new(Decimal::from(100), Currency::USD);
        let provider = MockRateProvider;
        let result = m.convert_to(Currency::USD, &provider).unwrap();
        assert_eq!(result.amount, Decimal::from(100));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn money_convert_usd_to_nio() {
        let m = Money::new(Decimal::from(100), Currency::USD);
        let provider = MockRateProvider;
        let result = m.convert_to(Currency::NIO, &provider).unwrap();
        assert_eq!(result.amount, Decimal::from(3600)); // 100 USD * 36 = 3600 NIO
        assert_eq!(result.currency, Currency::NIO);
    }

    #[test]
    fn money_convert_nio_to_usd() {
        let m = Money::new(Decimal::from(3600), Currency::NIO);
        let provider = MockRateProvider;
        let result = m.convert_to(Currency::USD, &provider).unwrap();
        // 3600 * 0.0277 ≈ 99.72
        assert_eq!(result.currency, Currency::USD);
        assert!(result.amount < Decimal::from(100) && result.amount > Decimal::from(99));
    }
}
