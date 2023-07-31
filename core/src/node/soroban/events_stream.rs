use futures::{Stream};
use log::{info, debug};
use soroban_cli::rpc::{EventStart, EventType, GetEventsResponse};
use tokio::time::{Duration};
use jsonrpsee_core::{params::ObjectParams, client::ClientT};
use jsonrpsee_http_client::{HeaderMap, HttpClient, HttpClientBuilder};

use crate::EventsStream;


const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn client(url: &str) -> HttpClient {
    let mut headers = HeaderMap::new();
    headers.insert("X-Client-Name", "soroban-cli".parse().unwrap());
    let version = VERSION.unwrap_or("devel");
    headers.insert("X-Client-Version", version.parse().unwrap());
    HttpClientBuilder::default()
        .set_headers(headers)
        .build(url).unwrap()
    
}

async fn get_events(
    rpc_url: &str,
    start: EventStart,
    event_type: Option<EventType>,
    contract_ids: &[String],
    topics: Option<&[String]>,
    limit: Option<usize>,
) -> Result<GetEventsResponse, serde_json::Error> {
    let mut filters = serde_json::Map::new();

    event_type
        .and_then(|t| match t {
            EventType::All => None, // all is the default, so avoid incl. the param
            EventType::Contract => Some("contract"),
            EventType::System => Some("system"),
        })
        .map(|t| filters.insert("type".to_string(), t.into()));

    filters.insert("topics".to_string(), topics.into());
    filters.insert("contractIds".to_string(), contract_ids.into());

    let mut pagination = serde_json::Map::new();
    if let Some(limit) = limit {
        pagination.insert("limit".to_string(), limit.into());
    }

    let mut oparams = ObjectParams::new();
    match start {
        EventStart::Ledger(l) => oparams.insert("startLedger", l.to_string())?,
        EventStart::Cursor(c) => {
            pagination.insert("cursor".to_string(), c.into());
        }
    };
    oparams.insert("filters", vec![filters])?;
    oparams.insert("pagination", pagination)?;

    Ok(client(rpc_url).request("getEvents", oparams).await.unwrap())
}


impl<'a> EventsStream<'a>    
    {
        pub fn stream(&self, poll_interval:Duration) -> impl Stream<Item = std::vec::Vec<soroban_cli::rpc::Event>> + '_{
            let configs = &self.config;
            let current_ledger = configs.starting_ledger;
            

            futures::stream::unfold(current_ledger, move |current_ledger: u32| async move {
                tokio::time::sleep(poll_interval).await;
            
                let event_start = EventStart::Ledger(current_ledger);
                
                let items = get_events(
                    configs.rpc_endpoint,
                    event_start, 
                    Some(EventType::Contract), 
                    &[configs.contract_id.to_string()], 
                    configs.topics, 
                    None
                ).await.unwrap(); // TODO: error handling.

                Some((items.events, items.latest_ledger)) // the core should always fecth for latest ledger 
                                                          // without caring to filter out duplicates.
                                                          // Duplicate filtering should take place when building
                                                          // out the database.
            })
        }
    }
