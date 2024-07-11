use alloy_rust::consts;
use alloy_rust::cron_util;
use eyre::{Ok, Result};
use alloy_rust::event_fetcher::event_handler::HttpBlocksMonitor;
#[tokio::main]
async fn main() -> Result<()> {
    // cron_util::create_cronjob_with_schedule(consts::CRON_EXPRESSION_15_SEC, check);
    let event_fetcher = HttpBlocksMonitor::new("https://eth.merkle.io").await?;
   let x =  event_fetcher.get_events("0xdAC17F958D2ee523a2206206994597C13D831ec7", "Transfer(address,address,uint256)").await?;
    Ok(())
}

fn check() {
    println!("Checking");
}
