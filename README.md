# `nyaa-si`

This API is an Unofficial Nyaa API rewritten in Rust.

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