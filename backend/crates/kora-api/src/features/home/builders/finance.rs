use kora_domain::finance::budget::Budget;
use crate::features::home::dto::{FinancePreview, FinanceAlert};

pub fn build(budgets: &[Budget]) -> FinancePreview {
    let total_budget: rust_decimal::Decimal = budgets.iter().map(|b| b.baseline().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let total_spent: rust_decimal::Decimal = budgets.iter().map(|b| b.current_expenses().amount).fold(rust_decimal::Decimal::from(0), |a, b| a + b);
    let pct = if total_budget.is_zero() { 0 } else { ((total_spent / total_budget) * rust_decimal::Decimal::from(100)).try_into().unwrap_or(0u32) };

    let alerts: Vec<FinanceAlert> = budgets.iter().filter_map(|b| {
        let v = b.get_variance().ok()?;
        if v.amount.is_sign_positive() {
            Some(FinanceAlert {
                cycle: b.cycle_id().0.clone(),
                text: format!("Presupuesto excedido por {} {:?}", v.amount, v.currency),
            })
        } else {
            let used_pct = if b.baseline().amount.is_zero() { 0 } else {
                ((b.current_expenses().amount / b.baseline().amount) * rust_decimal::Decimal::from(100)).try_into().unwrap_or(0u32)
            };
            if used_pct > 80 {
                Some(FinanceAlert { cycle: b.cycle_id().0.clone(), text: format!("Presupuesto al {}%", used_pct) })
            } else { None }
        }
    }).collect();

    FinancePreview {
        total_budget: format!("${}", total_budget),
        budget_used_percent: pct,
        total_spent: format!("${}", total_spent),
        alerts,
    }
}
