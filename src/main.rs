use alloy::rpc;
use alloy_rust::cron_util_2;
use alloy_rust::event_fetcher::event_handler::HttpBlocksMonitor;
use dotenv::dotenv;
use eyre::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC")?;
    let contract_address = env::var("CONTRACT_ADDR")?;
    let event_name = "Transfer(address,address,uint256)".to_owned();

    cron_util_2::create_cronjob_with_schedule(10, move || {
        let rpc_url = rpc_url.clone();
        let contract_address = contract_address.clone();
        let event_name = event_name.clone();
        async move {
            check2(
                rpc_url.clone(),
                contract_address.clone(),
                event_name.clone(),
            )
            .await
        }
    })
    .await;

    Ok(())
}
async fn check2(rpc_url: String, contract_address: String, event_name: String) {
    match HttpBlocksMonitor::new(&rpc_url).await {
        Ok(event_fetcher) => {
            match event_fetcher
                .get_events(&contract_address, &event_name)
                .await
            {
                Ok(events) => {
                    println!("Events fetched successfully: {:?}", events);
                }
                Err(e) => {
                    eprintln!("Failed to fetch events: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize HttpBlocksMonitor: {:?}", e);
        }
    }
}

