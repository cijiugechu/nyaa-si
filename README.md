# `nyaa-si`

[![Cargo](https://img.shields.io/crates/v/nyaa-si.svg)](https://crates.io/crates/nyaa-si) [![Documentation](https://docs.rs/nyaa-si/badge.svg)](https://docs.rs/nyaa-si)

An async Nyaa client for Rust.

## Usage

```rust
use nyaa_si::{Client, NyaaCategory, NyaaClient, QueryBuilder, Sort};

#[tokio::main]
async fn main() {
    let query = QueryBuilder::new()
        .search("tengoku")
        .sort(Sort::Downloads)
        .category(NyaaCategory::Anime)
        .build();

    let client = NyaaClient::new();
    let res = client.get(&query).await.unwrap();
    println!("{:#?}", res.get(0).unwrap());
}
```