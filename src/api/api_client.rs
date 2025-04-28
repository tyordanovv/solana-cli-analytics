use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use crate::{
    models::*,
    error::ApiError,
};
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_default();
            
        Self {
            client,
            base_url,
            api_key,
        }
    }
    
    pub async fn get_performance_samples(&self) -> Result<Vec<PerformanceSample>, ApiError> {
        let params = Some(vec![serde_json::json!(10)]);
        
        self.make_request("getRecentPerformanceSamples", params).await
    }
    
    pub async fn get_block_time(&self, slot: u64) -> Result<BlockTime, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_vote_accounts(&self) -> Result<VoteAccounts, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_fees(&self) -> Result<Fees, Box<dyn Error>> {
        todo!()
    }
    
    pub async fn get_priority_fee(&self) -> Result<PriorityFee, Box<dyn Error>> {
        todo!()
    }

    fn build_request_payload(&self, method: &str, params: Option<Vec<serde_json::Value>>) -> serde_json::Value {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params.unwrap_or_default()
        })
    }

        
    async fn make_request<T: for<'de> Deserialize<'de>>(
        &self, 
        method: &str, 
        params: Option<Vec<serde_json::Value>>
    ) -> Result<T, ApiError> {
        let url = format!("{}?api-key={}", self.base_url, self.api_key);
        print!("Making request to: {} \n", url);
        let client = &self.client;
        
        let payload = self.build_request_payload(method, params);
        
        // Send the POST request with API key in header
        let response = client.post(&url)
            .header("x-api-key", &self.api_key)
            .json(&payload)
            .send()
            .await?;
        
        // Check if the request was successful
        if !response.status().is_success() {
            return Err(ApiError::ResponseError(
                format!("API request failed: {}", response.status())
            ));
        }
        
        let json_response: serde_json::Value = response.json().await?;
        print!("Response: {:?} \n", json_response);

        if let Some(error) = json_response.get("error") {
            let code = error.get("code")
                .and_then(|c| c.as_i64())
                .unwrap_or(-1) as i32;
                
            let message = error.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            print!("Error: {} \n", message);
                
            return Err(ApiError::RpcError { code, message });
        }
        
        let result = json_response.get("result")
            .ok_or_else(|| ApiError::ResponseError("Missing 'result' field in response".to_string()))?;
        print!("Result: {:?} \n", result);
        let typed_result: T = serde_json::from_value(result.clone())?;
        Ok(typed_result)
    }
}