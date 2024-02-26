use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{fmt::Display, str::FromStr};

use crate::error::Error;

/// file size
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Size {
    KB(f32),
    MB(f32),
    GB(f32),
    TB(f32),
}

impl FromStr for Size {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, unit) = s.split_once(' ').ok_or_else(|| {
            Error::SizeParsingError(format!("Invalid size: {}", s))
        })?;

        let value = value.parse::<f32>().map_err(|_| {
            Error::SizeParsingError(format!("Invalid size: {}", s))
        })?;

        Ok(match unit {
            "KiB" => Size::KB(value),
            "MiB" => Size::MB(value),
            "GiB" => Size::GB(value),
            "TiB" => Size::TB(value),
            _ => {
                return Err(Error::SizeParsingError(format!(
                    "Invalid size: {}",
                    s
                )))
            }
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::KB(value) => write!(f, "{:.1} KiB", value),
            Size::MB(value) => write!(f, "{:.1} MiB", value),
            Size::GB(value) => write!(f, "{:.1} GiB", value),
            Size::TB(value) => write!(f, "{:.1} TiB", value),
        }
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs_in_kb = match self {
            Size::KB(value) => *value,
            Size::MB(value) => value * 1024.0,
            Size::GB(value) => value * 1024.0 * 1024.0,
            Size::TB(value) => value * 1024.0 * 1024.0 * 1024.0,
        };

        let rhs_in_kb = match other {
            Size::KB(value) => *value,
            Size::MB(value) => value * 1024.0,
            Size::GB(value) => value * 1024.0 * 1024.0,
            Size::TB(value) => value * 1024.0 * 1024.0 * 1024.0,
        };

        lhs_in_kb.partial_cmp(&rhs_in_kb)
    }
}

/// type definition for torrent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub title: String,
    pub link: String,
    pub magnet_url: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub seeders: u32,
    pub leechers: u32,
    pub downloads: u32,
    pub size: Size,
}

impl PartialEq for Torrent {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_size_units() {
        assert_eq!(Size::from_str("1 KiB").unwrap(), Size::KB(1.0));
        assert_eq!(Size::from_str("1 MiB").unwrap(), Size::MB(1.0));
        assert_eq!(Size::from_str("1 GiB").unwrap(), Size::GB(1.0));
        assert_eq!(Size::from_str("1 TiB").unwrap(), Size::TB(1.0));
    }

    #[test]
    fn display_size() {
        assert_eq!(Size::KB(1.2).to_string(), "1.2 KiB");
        assert_eq!(Size::MB(33.04).to_string(), "33.0 MiB");
        assert_eq!(Size::GB(1.0).to_string(), "1.0 GiB");
        assert_eq!(Size::TB(1.0).to_string(), "1.0 TiB");
    }

    #[test]
    fn compare_size() {
        assert!(Size::KB(1.0) < Size::MB(1.0));
        assert!(Size::MB(1.0) < Size::GB(1.0));
        assert!(Size::GB(1.0) < Size::TB(1.0));

        assert!(Size::KB(33.4) < Size::MB(44.5));
        assert!(Size::MB(33.4) < Size::GB(44.5));
        assert!(Size::GB(33.4) < Size::GB(44.5));
        assert!(Size::TB(33.4) < Size::TB(44.5));
    }
}
