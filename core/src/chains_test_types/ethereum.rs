use std::pin::Pin;
use async_trait::async_trait;
use web3::types::Filter;
use web3::futures::StreamExt;

use crate::EventLogger;

#[derive(Clone)]
pub struct MyListener {
    filter: Filter
}

#[async_trait]
impl EventLogger<std::result::Result<web3::types::Log, web3::Error>> for MyListener
    {
    fn new(contract_address: &[u8]) -> Self {
        let eth_filter = web3::types::FilterBuilder::default()
        .address(vec![web3::types::H160(contract_address.try_into().unwrap())])
        .topics(
            Some(vec![hex_literal::hex!(
                "d282f389399565f3671145f5916e51652b60eee8e5c759293a2f5771b8ddfd2e"
            )
            .into()]),
            None,
            None,
            None,
        )
        .build();

        Self { filter: eth_filter }
    }

    async fn read_stream(&self, poll_interval: std::time::Duration) -> Pin<Box<(dyn futures::Stream<Item = std::result::Result<web3::types::Log, web3::Error>> + Send + 'static)>> {
        let web3 = web3::Web3::new(web3::transports::Http::new("http://localhost:8545").unwrap());
        let filter = web3.eth_filter().create_logs_filter(self.filter.clone()).await;
        filter.unwrap().stream(poll_interval).boxed()
    }
}
