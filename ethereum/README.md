# Ethereum bridge

This is still a work in progress. This crate will implement a node that relies on `core` to enable atomic swaps from ethereum to soroban. Currently, the node doesn't perform any price calculations and will swap an equal amount of assets (so the swap happens between `n` token X on ethereum and `n` token X on soroban).

In the future, nodes could also perform price calculations between assets themselves and allow for true cross-chain swaps.
