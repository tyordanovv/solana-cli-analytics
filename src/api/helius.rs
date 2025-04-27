use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use crate::models::*;

pub struct HeliusClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl HeliusClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.helius.xyz/v1".to_string(),
            api_key,
        }
    }
    
    pub async fn get_performance_samples(&self) -> Result<Vec<PerformanceSample>, Box<dyn Error>> {
        todo!()
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
}