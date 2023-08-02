# Cross-Chain Bridge
#### Connecting other smart contract platforms to Soroban.

This is an experimental project built on top of `soroban-cortex-core` which aims to connect any smart contract platform to Soroban by implementing cross-chain swaps abilitated by message relaying oracles.

# Rationale

Whether it's about skepticism regarding centralized solutions or security concerns for decentralized ones, cross-chain bridges have always been a big question mark in the blockchain space.

In our approach to cross chain bridges we try to sacrifice liquidity efficiency for security and decentralization as much as possible without leaving the implementation factually liquidity inefficient.


We believe that on the generic user's side the implementation should be 100% decentralized between the two chains, while partially decentralized for liquidity providers. The bridge implementation in fact relies on liquidity providers deploying message relaying oracles themselves or accepting external oracles with a given weight and on their own terms. On the other hand, users can be completely agnostic of the various oracles and the in-between process. 


We understand that this implicates the need for more trust between liquidity providers (and oracles when they're not LP-owned). We also recognize that this solution might be safer for liquidity providers to run completely centralized, but on the user's side the solution remains completely trustless and decentralized. 


# Implementation

The implementation on both core and the actual bridge is currently broken and needs a lot of work to make this project production ready and not only experimental.

We are currently treating this project as an experiment, but it would be part of a potential submission for an SCF grant to make it production ready. What do you think? Let us know [here](https://github.com/xycloo/rs-soroban-cortex/discussions/1).
