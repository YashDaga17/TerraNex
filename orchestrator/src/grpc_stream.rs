use tracing::info;

pub async fn run() {
    info!("yellowstone stream idle: set feature `yellowstone` and wire endpoint envs to stream accounts");
    #[cfg(feature = "yellowstone")]
    {
        if let Err(err) = run_yellowstone().await {
            tracing::warn!("yellowstone stream stopped: {err}");
        }
    }
}

#[cfg(feature = "yellowstone")]
async fn run_yellowstone() -> anyhow::Result<()> {
    use futures_util::{SinkExt, StreamExt};
    use std::collections::HashMap;
    use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
    use yellowstone_grpc_proto::geyser::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts,
    };

    let endpoint = std::env::var("YELLOWSTONE_ENDPOINT")?;
    let token = std::env::var("YELLOWSTONE_TOKEN").ok();
    let program_id = std::env::var("NEXUS_PROGRAM_ID")?;

    let mut client = GeyserGrpcClient::build_from_shared(endpoint)?
        .x_token(token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;

    let mut accounts = HashMap::new();
    accounts.insert(
        "nexus-program".to_string(),
        SubscribeRequestFilterAccounts {
            owner: vec![program_id],
            ..Default::default()
        },
    );

    let request = SubscribeRequest {
        accounts,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    };

    let (mut tx, mut stream) = client.subscribe().await?;
    tx.send(request).await?;

    while let Some(update) = stream.next().await {
        if let Some(UpdateOneof::Account(account)) = update?.update_oneof {
            info!(
                "account update slot={} pubkey={:?}",
                account.slot, account.account
            );
        }
    }

    Ok(())
}
