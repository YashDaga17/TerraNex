use crate::SignedIntent;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Quote {
    pub intent_id: Uuid,
    pub solver: String,
    pub execution_price: u64,
    pub filled_size: u64,
    pub slot_hint: u64,
}

pub async fn run_auction(intent: &SignedIntent) -> Quote {
    let mut best = Quote {
        intent_id: intent.id,
        solver: "solver-local".to_string(),
        execution_price: intent.limit_price,
        filled_size: intent.size,
        slot_hint: intent.expires_slot.saturating_sub(1),
    };

    for step in 1..=5 {
        sleep(Duration::from_millis(300)).await;
        let improvement = intent
            .limit_price
            .saturating_mul(step)
            .saturating_div(10_000);
        best.execution_price = match intent.side.as_str() {
            "short" => intent.limit_price.saturating_add(improvement),
            _ => intent.limit_price.saturating_sub(improvement),
        };
    }

    best
}
