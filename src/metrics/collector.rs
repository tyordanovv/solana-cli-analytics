use std::sync::{Arc, Mutex};
use tokio::time::Duration;
use crate::api::api_client::ApiClient;
use crate::websocket::ws_client::{WebSocketClient, WebSocketEventHandler};
use crate::metrics::aggregator::NumericTimeSeries;


pub struct MetricsCollector {
    http_client: Arc<ApiClient>,
    ws_client: Arc<WebSocketClient>,
    tps_metrics: Arc<Mutex<NumericTimeSeries<>>>,
    latency_metrics: Arc<Mutex<NumericTimeSeries<>>>,
    fee_metrics: Arc<Mutex<NumericTimeSeries<>>>,
    validator_metrics: Arc<Mutex<std::collections::HashMap<String, NumericTimeSeries<>>>>,
}

impl MetricsCollector {
    pub fn new(http_client: Arc<ApiClient>, ws_client: Arc<WebSocketClient>) -> Self {
        Self {
            http_client,
            ws_client,
            tps_metrics: Arc::new(Mutex::new(NumericTimeSeries::new(1000))),
            latency_metrics: Arc::new(Mutex::new(NumericTimeSeries::new(1000))),
            fee_metrics: Arc::new(Mutex::new(NumericTimeSeries::new(1000))),
            validator_metrics: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }
    
    pub async fn start_collection(&self, poll_interval: Duration) {
        todo!()
    }
    
    pub async fn collect_performance(&self) {
        todo!()
    }
    
    pub async fn collect_fees(&self) {
        todo!()
    }
    
    pub async fn collect_validators(&self) {
        todo!()
    }
}

impl WebSocketEventHandler for MetricsCollector {
    fn on_slot_update(&self, slot: u64) {
        println!("Slot update: {}", slot);
    }
    
    fn on_signature_update(&self, signature: String, status: String) {
        println!("Signature {} update: {}", signature, status);
    }
    
    fn on_root_update(&self, root: u64) {
        println!("Root update: {}", root);
    }
    
    fn on_error(&self, error: String) {
        eprintln!("WebSocket error: {}", error);
    }
}