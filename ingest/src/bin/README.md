# Examples

Below there are ingestion examples you can achieve with `ingestion`.

> Note: This crate is still a work in progress (in fact it was started on Aug 8). The current capabilities of the crate are limited. 

## Hello ledger!

This simple example shows how to capture a ledger that is present in the archiver, so not the current futurenet ledger for example.


```rust
use ingest::{IngestionConfig, CaptiveCore, Range, BoundedRange};
use stellar_xdr::next::LedgerCloseMeta;

pub fn main() {
    let config = IngestionConfig {
        executable_path: "/usr/local/bin/stellar-core".to_string(), // path to stellar-core executable
        context_path: Default::default(),
    };

    let mut captive_core = CaptiveCore::new(config);

    let range = Range::Bounded(BoundedRange(292395, 292396));
    captive_core.prepare_ledgers(range).unwrap();

    let ledger = captive_core.get_ledger(292395);
    let ledger_seq = match ledger.unwrap() {
        LedgerCloseMeta::V1(v1) => v1.ledger_header.header.ledger_seq,
        _ => unreachable!() // remember that this is just an example, you might need to handle different scenarios.
    };

    println!("Hello ledger {}", ledger_seq);
}
```

Here, we first prepare the range of ledgers we're interested in and then get a ledger of our choice. Then, we grab the sequence number of the ledger we captured and print an hello world message ensuring that the ingestion worked correctly. 
