use tui::Terminal;
use tui::backend::Backend;
use tui::layout::Rect;
use std::sync::Arc;
use std::time::Duration;
use crate::metrics::collector::MetricsCollector;

pub struct DashboardConfig {
    pub refresh_rate: Duration,
    pub window_size: Duration,
    pub show_cluster_health: bool,
    pub show_fees: bool,
    pub show_validator_stats: bool,
    pub show_token_analytics: bool
}

pub struct Dashboard<B: Backend> {
    terminal: Terminal<B>,
    metrics_collector: Arc<MetricsCollector>,
    config: DashboardConfig,
}

impl<B: Backend> Dashboard<B> {
    pub fn new(
        terminal: Terminal<B>, 
        metrics_collector: Arc<MetricsCollector>,
        config: DashboardConfig,
    ) -> Self {
        Self {
            terminal,
            metrics_collector,
            config,
        }
    }
    
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
    
    fn render_cluster_health(&self, area: Rect) -> Result<(), Box<dyn std::error::Error>> {
        // get_health
        // get_epoch_info
        // get_performance_samples
        // get_stake_activation
        // get_block_time
        // ws slotUpdate
        todo!()
    }
    
    fn render_fees(&self, area: Rect) -> Result<(), Box<dyn std::error::Error>> {
        // get_health
        // get_fees
        // get_priority_fee
        todo!()
    }
    
    fn render_validator_stats(&self, area: Rect) -> Result<(), Box<dyn std::error::Error>> {
        // get_health
        // get_vote_accounts
        // get_epoch_info
        todo!()
    }

    fn render_token_analytics(&self, area: Rect) -> Result<(), Box<dyn std::error::Error>> {
        // get_health
        // get_token_largest_accounts
        // get_token_supply
        // get_account_info
        todo!()
    }
}