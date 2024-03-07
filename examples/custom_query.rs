use nyaa_si::{Client, NyaaCategory, NyaaClient, QueryBuilder, Sort};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let query = QueryBuilder::new()
        .search("tengoku")
        .sort(Sort::Downloads)
        .category(NyaaCategory::Anime)
        .build();

    let client = NyaaClient::new();
    let res = client.get(&query).await.unwrap();
    println!("{:#?}", res.get(0).unwrap());
}
