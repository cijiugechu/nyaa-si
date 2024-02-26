use std::sync::OnceLock;

use scraper::{ElementRef, Html, Selector};

use crate::error::{Error, Result};
use crate::model::{Size, Torrent};

static ITEM_SELECTOR: OnceLock<Selector> = OnceLock::new();
static TITLE_SELECTOR: OnceLock<Selector> = OnceLock::new();
static TORRENT_LINK_SELECTOR: OnceLock<Selector> = OnceLock::new();
static MAGNET_SELECTOR: OnceLock<Selector> = OnceLock::new();
static SEEDERS_SELECTOR: OnceLock<Selector> = OnceLock::new();
static LEECHERS_SELECTOR: OnceLock<Selector> = OnceLock::new();
static DATE_SELECTOR: OnceLock<Selector> = OnceLock::new();
static DOWNLOADS_SELECTOR: OnceLock<Selector> = OnceLock::new();
static SIZE_SELECTOR: OnceLock<Selector> = OnceLock::new();

pub fn extract(html: &str, base_url: &str) -> Result<Vec<Torrent>> {
    let document = Html::parse_document(html);
    let selector = ITEM_SELECTOR
        .get_or_init(|| Selector::parse("table>tbody>tr").unwrap());
    let items = document.select(selector);
    let mut res_vec: Vec<Torrent> = vec![];

    for item in items {
        let title = extract_title(item)?;
        let torrent_link = extract_torrent_link(item, base_url)?;
        let magnet = extract_magnet_url(item)?;
        let seeders = extract_seeders(item)?;
        let leechers = extract_leechers(item)?;
        let downloads = extract_downloads(item)?;
        let size = extract_size(item)?;
        let date = extract_date(item)?;
        let torrent = Torrent {
            title,
            link: torrent_link,
            magnet_url: magnet,
            date,
            seeders,
            leechers,
            downloads,
            size,
        };
        res_vec.push(torrent);
    }
    Ok(res_vec)
}

fn extract_title(item: ElementRef<'_>) -> Result<String> {
    let selector = TITLE_SELECTOR.get_or_init(|| {
        Selector::parse("td:nth-of-type(2)>a:last-child").unwrap()
    });
    let title = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Title not found".into()))?;
    Ok(title.text().collect())
}

fn extract_torrent_link(
    item: ElementRef<'_>,
    base_url: &str,
) -> Result<String> {
    let selector = TORRENT_LINK_SELECTOR.get_or_init(|| {
        Selector::parse("td:nth-of-type(3)>a:first-child").unwrap()
    });
    let link = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Link not found".into()))?;
    link.value()
        .attr("href")
        .ok_or(Error::SelectorError("Link not found".into()))
        .map(|s| format!("{}{}", base_url, s))
}

fn extract_magnet_url(item: ElementRef<'_>) -> Result<String> {
    let selector = MAGNET_SELECTOR.get_or_init(|| {
        Selector::parse("td:nth-of-type(3)>a:last-child").unwrap()
    });
    let link = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("magnet not found".into()))?;
    link.value()
        .attr("href")
        .ok_or(Error::SelectorError("magnet not found".into()))
        .map(|s| s.to_string())
}

fn extract_seeders(item: ElementRef<'_>) -> Result<u32> {
    let selector = SEEDERS_SELECTOR
        .get_or_init(|| Selector::parse("td:nth-of-type(6)").unwrap());
    let seeders = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Seeders not found".into()))?;
    let seeders_str: String = seeders.text().collect();
    seeders_str
        .parse::<u32>()
        .map_err(|_| Error::SelectorError("Seeders not found".into()))
}

fn extract_leechers(item: ElementRef<'_>) -> Result<u32> {
    let selector = LEECHERS_SELECTOR
        .get_or_init(|| Selector::parse("td:nth-of-type(7)").unwrap());
    let leechers = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Leechers not found".into()))?;
    let leechers_str: String = leechers.text().collect();
    leechers_str
        .parse::<u32>()
        .map_err(|_| Error::SelectorError("Leechers not found".into()))
}

fn extract_downloads(item: ElementRef<'_>) -> Result<u32> {
    let selector = DOWNLOADS_SELECTOR
        .get_or_init(|| Selector::parse("td:nth-of-type(8)").unwrap());
    let downloads = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Downloads not found".into()))?;
    let downloads_str: String = downloads.text().collect();
    downloads_str
        .parse::<u32>()
        .map_err(|_| Error::SelectorError("Downloads not found".into()))
}

fn extract_size(item: ElementRef<'_>) -> Result<Size> {
    let selector = SIZE_SELECTOR
        .get_or_init(|| Selector::parse("td:nth-of-type(4)").unwrap());
    let size = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Size not found".into()))?;
    size.text().collect::<String>().parse()
}

fn extract_date(item: ElementRef<'_>) -> Result<chrono::DateTime<chrono::Utc>> {
    let selector = DATE_SELECTOR
        .get_or_init(|| Selector::parse("td:nth-of-type(5)").unwrap());
    let date = item
        .select(selector)
        .next()
        .ok_or(Error::SelectorError("Date not found".into()))?;
    date.attr("data-timestamp")
        .ok_or(Error::SelectorError("Date not found".into()))
        .map(|s| {
            let time_stamp = s.parse::<i64>().unwrap();
            let ts_in_millis = time_stamp * 1000;
            chrono::DateTime::from_timestamp_millis(ts_in_millis)
                .unwrap_or_default()
        })
}
