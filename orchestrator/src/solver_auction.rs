use crate::SignedIntent;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Quote {
    pub event: &'static str,
    pub intent_id: Uuid,
    pub solver: String,
    pub execution_price: u64,
    pub filled_size: u64,
    pub slot_hint: u64,
    pub step: u8,
}

pub fn quote_for_step(intent: &SignedIntent, step: u8) -> Quote {
    let step = step.clamp(1, 5);
    let improvement = intent
        .limit_price
        .saturating_mul(step as u64)
        .saturating_div(10_000);

    Quote {
        event: if step == 5 {
            "final_quote"
        } else {
            "solver_quote"
        },
        intent_id: intent.id,
        solver: "solver-local".to_string(),
        execution_price: match intent.side.as_str() {
            "short" => intent.limit_price.saturating_add(improvement),
            _ => intent.limit_price.saturating_sub(improvement),
        },
        filled_size: intent.size,
        slot_hint: intent.expires_slot.saturating_sub(1),
        step,
    }
}
