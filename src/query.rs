use std::{fmt::Display, str::FromStr};

use crate::category::Category;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum SortOrder {
    Asecending,
    #[default]
    Descending,
}

impl FromStr for SortOrder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asc" => Ok(SortOrder::Asecending),
            "desc" => Ok(SortOrder::Descending),
            _ => Err("Invalid sort order".into()),
        }
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asecending => write!(f, "asc"),
            SortOrder::Descending => write!(f, "desc"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum Sort {
    Comments,
    Size,
    Date,
    #[default]
    Seeders,
    Leechers,
    Downloads,
}

impl FromStr for Sort {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "comments" => Ok(Sort::Comments),
            "size" => Ok(Sort::Size),
            "date" => Ok(Sort::Date),
            "seeders" => Ok(Sort::Seeders),
            "leechers" => Ok(Sort::Leechers),
            "downloads" => Ok(Sort::Downloads),
            _ => Err("Invalid sort".into()),
        }
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sort::Comments => write!(f, "comments"),
            Sort::Size => write!(f, "size"),
            Sort::Date => write!(f, "date"),
            Sort::Seeders => write!(f, "seeders"),
            Sort::Leechers => write!(f, "leechers"),
            Sort::Downloads => write!(f, "downloads"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum Filter {
    #[default]
    NoFilter,
    NoRemakes,
    TrustedOnly,
}

impl From<u8> for Filter {
    fn from(i: u8) -> Self {
        match i {
            0 => Filter::NoFilter,
            1 => Filter::NoRemakes,
            2 => Filter::TrustedOnly,
            _ => Filter::NoFilter,
        }
    }
}

impl From<Filter> for u8 {
    fn from(i: Filter) -> Self {
        match i {
            Filter::NoFilter => 0,
            Filter::NoRemakes => 1,
            Filter::TrustedOnly => 2,
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::NoFilter => write!(f, "0"),
            Filter::NoRemakes => write!(f, "1"),
            Filter::TrustedOnly => write!(f, "2"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash)]
pub struct Query<C: Category> {
    search: String,
    page: u32,
    sort: Sort,
    sort_order: SortOrder,
    filter: Filter,
    category: C,
}

impl<C> Display for Query<C>
where
    C: Category,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "q={}&p={}&s={}&o={}&f={}&c={}",
            self.search,
            self.page,
            self.sort,
            self.sort_order,
            self.filter,
            self.category
        )
    }
}

impl<C> Default for Query<C>
where
    C: Category,
{
    fn default() -> Self {
        Query {
            search: String::new(),
            page: 1,
            sort: Sort::default(),
            sort_order: SortOrder::default(),
            filter: Filter::default(),
            category: C::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryBuilder<C: Category> {
    search: String,
    page: u32,
    sort: Sort,
    sort_order: SortOrder,
    filter: Filter,
    category: C,
}

impl<C: Category> Default for QueryBuilder<C> {
    fn default() -> Self {
        QueryBuilder {
            search: String::new(),
            page: 1,
            sort: Sort::default(),
            sort_order: SortOrder::default(),
            filter: Filter::default(),
            category: C::default(),
        }
    }
}

impl<C: Category> QueryBuilder<C> {
    pub fn new() -> QueryBuilder<C> {
        QueryBuilder::default()
    }

    pub fn build(self) -> Query<C> {
        Query {
            search: self.search,
            page: self.page,
            sort: self.sort,
            sort_order: self.sort_order,
            filter: self.filter,
            category: C::default(),
        }
    }

    pub fn search<S: Into<String>>(mut self, search: S) -> QueryBuilder<C> {
        self.search = search.into();
        self
    }

    pub fn page(mut self, page: u32) -> QueryBuilder<C> {
        self.page = page;
        self
    }

    pub fn sort(mut self, sort: Sort) -> QueryBuilder<C> {
        self.sort = sort;
        self
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> QueryBuilder<C> {
        self.sort_order = sort_order;
        self
    }

    pub fn filter(mut self, filter: Filter) -> QueryBuilder<C> {
        self.filter = filter;
        self
    }

    pub fn category(mut self, category: C) -> QueryBuilder<C> {
        self.category = category;
        self
    }
}
