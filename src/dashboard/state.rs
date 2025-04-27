use std::collections::HashMap;

pub struct AppState {
    selected_tab: usize,
    validator_filter: Option<String>,
    metrics_config: HashMap<String, bool>, 
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
            validator_filter: None,
            metrics_config: HashMap::new(),
        }
    }
    
    pub fn next_tab(&mut self) {
        todo!()
    }
    
    pub fn prev_tab(&mut self) {
        todo!()
    }
    
    pub fn set_validator_filter(&mut self, pubkey: Option<String>) {
        self.validator_filter = pubkey;
    }
    
    pub fn toggle_metric(&mut self, metric_name: &str) {
        todo!()
    }
}