use alloy_rust::consts;
use alloy_rust::cron_util;
use eyre::{Ok, Result};
use alloy_rust::event_fetcher::event_handler::HttpBlocksMonitor;
#[tokio::main]
async fn main() -> Result<()> {
    // cron_util::create_cronjob_with_schedule(consts::CRON_EXPRESSION_15_SEC, check);
    let event_fetcher = HttpBlocksMonitor::new("https://eth.merkle.io").await?;
    event_fetcher.get_events("", "YourEventName").await;
    Ok(())
}

fn check() {
    println!("Checking");
}
