use alloy::providers::{Provider, RootProvider};
use alloy::transports::http::Http;
use alloy::{
    primitives::{b256, Address},
    providers::ProviderBuilder,
    rpc::types::Filter,
    sol,
};
use dotenv::dotenv;
use eyre::{Ok, Result};
use reqwest::{Client, Url};

pub struct HttpBlocksMonitor {
    provider: RootProvider<Http<Client>>,
}

impl HttpBlocksMonitor {
    pub async fn new(rpc: &str) -> Result<Self> {
        let provider: RootProvider<Http<Client>> = ProviderBuilder::new().on_http(rpc.parse()?);

        Ok(HttpBlocksMonitor { provider: provider })
    }

    pub async fn get_events(&self, contract_addr: &str, event_name: &str) -> Result<()> {
        let current_block = self.provider.get_block_number().await?;
        // let mut transformed_logs = Vec::new();
        let mut end_block = 12;
        let start_block = 20287845;

        if current_block >= start_block {
            end_block = if start_block + 10 > current_block {
                current_block
            } else {
                start_block + 10
            };
        }
        println!("startBlock = {start_block} , end_block = {end_block} , Current_block = {current_block}" );

        let filter = Filter::new()
            .address(contract_addr.parse::<Address>()?)
            .event(event_name)
            .from_block(start_block)
            .to_block(end_block);
        let logs = self.provider.get_logs(&filter).await?;

        for log in logs {
            // println!(
            //     "---------------------------------------------------",);
            // println!("-----------------------{:?}----------------------------",  log.block_number.unwrap());
        }

        Ok(())
    }
}
