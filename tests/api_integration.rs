use solana_cli_analytics::api::api_client::ApiClient;
use std::env;
use tokio;
use dotenv::dotenv;

#[tokio::test]
async fn test_get_performance_samples() {
    dotenv().ok();

    let api_key = match env::var("HELIUS_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping API integration test: HELIUS_API_KEY not set");
            return;
        }
    };
    
    let client = ApiClient::new(
        api_key,
        "https://mainnet.helius-rpc.com/".to_string()
    );
    
    // Test basic connectivity
    let samples = client.get_performance_samples().await;
    assert!(samples.is_ok(), "Failed to get performance samples: {:?}", samples);
}