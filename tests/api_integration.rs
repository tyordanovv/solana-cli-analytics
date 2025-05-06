use solana_cli_analytics::api::api_client::ApiClient;
use std::env;
use dotenv::dotenv;

async fn setup_client() -> Option<ApiClient> {
    dotenv().ok();
    
    match env::var("HELIUS_API_KEY") {
        Ok(api_key) => Some(ApiClient::new(
            api_key,
            "https://mainnet.helius-rpc.com/".to_string()
        )),
        Err(_) => {
            println!("Skipping API integration test: HELIUS_API_KEY not set");
            None
        }
    }
}

#[tokio::test]
async fn test_get_performance_samples() {
    if let Some(client) = setup_client().await {
        let samples = client.get_performance_samples().await;
        assert!(samples.is_ok(), "Failed to get performance samples: {:?}", samples);
    }
}

#[tokio::test]
async fn test_get_health() {
    if let Some(client) = setup_client().await {
        let health = client.get_health().await;
        assert!(health.is_ok(), "Failed to get health: {:?}", health);
    }
}

#[tokio::test]
async fn test_get_epoch_info() {
    if let Some(client) = setup_client().await {
        let epoch_info = client.get_epoch_info().await;
        assert!(epoch_info.is_ok(), "Failed to get epoch info: {:?}", epoch_info);

        let epoch_info = epoch_info.unwrap();
        println!("Epoch Info: {:?}", epoch_info);

        assert!(epoch_info.epoch > 0, "Epoch should be greater than 0");
        assert!(epoch_info.slots_in_epoch > 0, "Slots in epoch should be greater than 0");
    }
}
