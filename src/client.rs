use crate::category::Category;
use crate::error::Result;
use crate::model::Torrent;
use crate::query::Query;

pub trait Client<C: Category> {
    const BASE_URL: &'static str;

    /// Send a query to the api
    ///
    ///```
    #[doc = include_str!("../examples/custom_query.rs")]
    ///```
    fn get(
        &self,
        query: &Query<C>,
    ) -> impl std::future::Future<Output = Result<Vec<Torrent>>> + Send;
}
