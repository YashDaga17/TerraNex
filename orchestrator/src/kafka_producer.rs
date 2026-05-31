use crate::SignedIntent;

#[cfg(feature = "kafka")]
pub async fn publish_intent(intent: &SignedIntent) {
    use rdkafka::producer::{FutureProducer, FutureRecord};
    use rdkafka::ClientConfig;
    use std::time::Duration;

    let Ok(brokers) = std::env::var("KAFKA_BROKERS") else {
        return;
    };
    let Ok(producer) = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create::<FutureProducer>()
    else {
        return;
    };

    let payload = match serde_json::to_string(intent) {
        Ok(payload) => payload,
        Err(_) => return,
    };

    let _ = producer
        .send(
            FutureRecord::to("nexus.intents")
                .key(&intent.id.to_string())
                .payload(&payload),
            Duration::from_millis(0),
        )
        .await;
}

#[cfg(not(feature = "kafka"))]
pub async fn publish_intent(_intent: &SignedIntent) {}
