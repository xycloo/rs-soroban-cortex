# rs-ingest
## Ingestion library written in Rust for futurenet

This package provides primitives for building custom ingestion engines on Futurenet. It's inspired from stellars [`go/ingest`](https://github.com/stellar/go/tree/master/ingest) package.

Often, developers either need ingestion features that are outside of Horizon's scope, or need higher availability for the data. For example, a protocol's frontend might need to ingest events into their own database with a very specific filtering or at a large amount, or they might also need to replay history to see if they missed some events. 

This crate is being designed with this need in mind, and works on futurenet!

> Note: This crate is still a work in progress (in fact it was started on Aug 8). The current capabilities of the crate are limited.

> Note: Currently only POSIX systems are supported.

# Features

Currently, only replaying history is supported, but unbounded ingesting will be added soon.

# Try it out

The crate is a WIP, but you can already start playing around the features it currently offers. For example, check out the [examples](https://github.com/xycloo/rs-soroban-cortex/tree/main/ingest/src/bin).

## Setup

Before using the crate, you need the [`stellar-core`](https://github.com/stellar/stellar-core) executable. To install the currently futurenet-compatible core:

```
git clone https://github.com/stellar/stellar-core

cd stellar-core

git checkout b7d3a8f8

git submodule init

git submodule update

./autogen

CXX=clang++-12 ./configure --enable-next-protocol-version-unsafe-for-production

make

make install [this one might need root access on some machines]
```
