use std::fmt::Display;
use tokio::task::spawn_blocking;
use tracing::{event, Level};

use crate::{
    category::Category, client::Client, error::Result, extractor::extract,
    model::Torrent, query::Query,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NyaaCategory {
    All,
    Anime,
    AnimeMusicVideo,
    AnimeEnglishTranslated,
    AnimeNonEnglishTranslated,
    AnimeRaw,
    Audio,
    AudioLossless,
    AudioLossy,
    Literature,
    LiteratureEnglishTranslated,
    LiteratureNonEnglishTranslated,
    LiteratureRaw,
    LiveAction,
    LiveActionEnglishTranslated,
    LiveActionIdol,
    LiveActionNonEnglishTranslated,
    LiveActionRaw,
    Pictures,
    PicturesGraphics,
    PicturesPhotos,
    Software,
    SoftwareApplications,
    SoftwareGames,
}

impl Display for NyaaCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "0_0"),
            Self::Anime => write!(f, "1_0"),
            Self::AnimeMusicVideo => write!(f, "1_1"),
            Self::AnimeEnglishTranslated => write!(f, "1_2"),
            Self::AnimeNonEnglishTranslated => write!(f, "1_3"),
            Self::AnimeRaw => write!(f, "1_4"),
            Self::Audio => write!(f, "2_0"),
            Self::AudioLossless => write!(f, "2_1"),
            Self::AudioLossy => write!(f, "2_2"),
            Self::Literature => write!(f, "3_0"),
            Self::LiteratureEnglishTranslated => write!(f, "3_1"),
            Self::LiteratureNonEnglishTranslated => write!(f, "3_2"),
            Self::LiteratureRaw => write!(f, "3_3"),
            Self::LiveAction => write!(f, "4_0"),
            Self::LiveActionEnglishTranslated => write!(f, "4_1"),
            Self::LiveActionIdol => write!(f, "4_2"),
            Self::LiveActionNonEnglishTranslated => write!(f, "4_3"),
            Self::LiveActionRaw => write!(f, "4_4"),
            Self::Pictures => write!(f, "5_0"),
            Self::PicturesGraphics => write!(f, "5_1"),
            Self::PicturesPhotos => write!(f, "5_2"),
            Self::Software => write!(f, "6_0"),
            Self::SoftwareApplications => write!(f, "6_1"),
            Self::SoftwareGames => write!(f, "6_2"),
        }
    }
}

impl Default for NyaaCategory {
    fn default() -> Self {
        Self::All
    }
}

impl Category for NyaaCategory {}

pub type NyaaQuery = Query<NyaaCategory>;

#[derive(Debug, Default)]
pub struct NyaaClient {
    inner: reqwest::Client,
}

impl NyaaClient {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Client<NyaaCategory> for NyaaClient {
    const BASE_URL: &'static str = "https://nyaa.si";

    /// Send a query to the api
    ///
    ///```
    #[doc = include_str!("../examples/custom_query.rs")]
    ///```
    #[tracing::instrument(skip(self))]
    async fn get(&self, query: &Query<NyaaCategory>) -> Result<Vec<Torrent>> {
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
        let client = NyaaClient::new();
        let query = QueryBuilder::new()
            .search("frieren")
            .sort(Sort::Downloads)
            .build();
        let res = client.get(&query).await.unwrap();

        println!("{:#?}", res.get(0).unwrap());
    }
}
