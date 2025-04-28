use clap::{Parser, ArgAction};
use std::error::Error;
use std::time::Duration;
use std::collections::HashMap;
use solana_cli_analytics::{
    api::api_client::ApiClient,
    websocket::ws_client::WebSocketClient,
    metrics::collector::MetricsCollector,
    dashboard::ui::{Dashboard, DashboardConfig},
    dashboard::state::AppState,
};
#[derive(Parser, Debug)]
#[command(
    name = "solana-cli-analytics",
    version,
    about = "Real-time Solana cluster analytics in your terminal",
    long_about = None
)]
struct CliArgs {
    #[arg(long, value_name = "RPC_API_KEY")]
    rpc_api_key: String,

    #[arg(
        long,
        default_value = "https://mainnet.helius-rpc.com/",
        value_name = "RPC_URL"
    )]
    rpc_url: String,

    #[arg(
        long,
        default_value = "wss://api.helius.xyz/v1/ws",
        value_name = "WS_URL"
    )]
    ws_url: String,

    #[arg(long, default_value = "1", value_name = "SECS")]
    refresh_rate: u64,

    #[arg(long, default_value = "60", value_name = "SECS")]
    window_size: u64,

    #[arg(long, action = ArgAction::SetTrue)]
    show_cluster_health: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    show_fees: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    show_validator_stats: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    show_rpc_health: bool,

    #[arg(long, action = ArgAction::SetTrue)]
    show_mempool: bool,
}

#[derive(Debug)]
struct Config {
    api_key: String,
    rpc_url: String,
    ws_url: String,
    refresh_rate: Duration,
    window_size: Duration,
    panels: HashMap<&'static str, bool>,
}

impl From<CliArgs> for Config {
    fn from(args: CliArgs) -> Self {
        let mut panels = HashMap::new();
        panels.insert("cluster_health", args.show_cluster_health);
        panels.insert("fees", args.show_fees);
        panels.insert("validator_stats", args.show_validator_stats);
        panels.insert("rpc_health", args.show_rpc_health);
        panels.insert("mempool", args.show_mempool);

        Config {
            api_key: args.rpc_api_key,
            rpc_url: args.rpc_url,
            ws_url: args.ws_url,
            refresh_rate: Duration::from_secs(args.refresh_rate),
            window_size: Duration::from_secs(args.window_size),
            panels,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = CliArgs::parse();
    let cfg: Config = cli.into();
    println!("Config: {:?}", cfg);

    let helius_api_client: ApiClient = ApiClient::new(cfg.api_key.clone(), cfg.rpc_url.clone());

    // 3. Set up the WebSocket client
    

    // 4. Create the metrics collector
    
    // 5. Initialize the dashboard
    let dashboard_config: DashboardConfig = DashboardConfig {
        refresh_rate: cfg.refresh_rate,
        window_size: cfg.window_size,
        show_cluster_health: cfg.panels["cluster_health"],
        show_fees: cfg.panels["fees"],
        show_validator_stats: cfg.panels["validator_stats"],
        show_rpc_health: cfg.panels["rpc_health"],
        show_mempool: cfg.panels["mempool"],
    };

    // 6. Start the metrics collection in the background
    
    // 7. Run the dashboard UI main loop
    
    // 8. Clean up resources when done
    
    Ok(())
}