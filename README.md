# Soroban Cortex

#### Crates to run soroban-related services.

Soroban Cortex is a project that aims to provide stable and secure crates that allow implementors to quickly set up services around Soroban contracts.

> Note: currently the only crate suitable for external usage in the workspace is [`soroban-events-streaming-node`](https://github.com/xycloo/rs-soroban-cortex/tree/main/soroban-events-streaming-node).

# Overview and Features

`rs-soroban-cortex/soroban-cortex-core` offers modules to quickly spin up Soroban-related services. At the time of writing this readme, the crate aims to offer:
- module(s) to build cross-chain messaging oracles [chain -> Soroban].
- module(s) to deal with Soroban events.

## Cross-chain messaging

Modules and objects regarding the cross-chain messaging functionality are currently broken and are the beginning of an experimental trustless multichain bridge to Soroban based on swaps. See more info [here](https://github.com/xycloo/rs-soroban-cortex/tree/main/cross-chain-bridge). 

## Soroban events streaming

While still a WIP, core provides the `soroban_cortex_core::EventsStream` object to stream events according to the provided stream configuration.


# Next steps
The whole workspace as is is very raw and not suitable for production projects. Luckily, Soroban hasn't launched yet which gives us plenty of time to work on Cortex.

In the next weeks, if we see we interest and have enough manpower/time at xyclooLabs we may also apply for the Stellar Community Fund grant to make this project production-ready and get the bridge(s) up and running. What do you think? Let us know [here](https://github.com/xycloo/rs-soroban-cortex/discussions/1).

# Built on Soroban Cortex Core

#### Soroban events streaming node

Currently, the only crate depending on core which, while still a WIP, is functional is [`soroban-events-streaming-node`](https://github.com/xycloo/rs-soroban-cortex/tree/main/soroban-events-streaming-node). This crate allows implementors to quickly set up a fairly customizable event streaming service (we see it being fully customizable in the future).


# Feature requests and contributing

Contributions are welcome. To suggest a feature open an issue on the repo.
