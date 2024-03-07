use std::fmt::Display;
use tokio::task::spawn_blocking;
use tracing::{event, Level};

use crate::{
    category::Category, client::Client, error::Result, extractor::extract,
    model::Torrent, query::Query,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SukebeiCategory {
    All,
    Art,
    ArtAnime,
    ArtDoujinshi,
    ArtGames,
    ArtManga,
    ArtPictures,
    RealLife,
    RealLifePhotobooks,
    RealLifeVideos,
}

impl Display for SukebeiCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "0_0"),
            Self::Art => write!(f, "1_0"),
            Self::ArtAnime => write!(f, "1_1"),
            Self::ArtDoujinshi => write!(f, "1_2"),
            Self::ArtGames => write!(f, "1_3"),
            Self::ArtManga => write!(f, "1_4"),
            Self::ArtPictures => write!(f, "1_5"),
            Self::RealLife => write!(f, "2_0"),
            Self::RealLifePhotobooks => write!(f, "2_1"),
            Self::RealLifeVideos => write!(f, "2_2"),
        }
    }
}

impl Default for SukebeiCategory {
    fn default() -> Self {
        Self::All
    }
}

impl Category for SukebeiCategory {}

pub type SukebeiQuery = Query<SukebeiCategory>;

#[derive(Debug, Default)]
pub struct SukebeiClient {
    inner: reqwest::Client,
}

impl SukebeiClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Client<SukebeiCategory> for SukebeiClient {
    const BASE_URL: &'static str = "https://sukebei.nyaa.si";

    #[tracing::instrument(skip(self))]
    async fn get(
        &self,
        query: &Query<SukebeiCategory>,
    ) -> Result<Vec<Torrent>> {
        let url = format!("{}/?{}", Self::BASE_URL, query);

        event!(Level::DEBUG, "url = {}", url);

        let res = self.inner.get(url).send().await?.text().await?;

        spawn_blocking(move || extract(&res, Self::BASE_URL)).await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryBuilder;
    use crate::query::Sort;

    #[tokio::test]
    async fn test_get() {
        let client = SukebeiClient::new();
        let query = QueryBuilder::new()
            .search("dl")
            .sort(Sort::Downloads)
            .build();
        let res = client.get(&query).await.unwrap();

        println!("{:#?}", res.get(0).unwrap());
    }
}
