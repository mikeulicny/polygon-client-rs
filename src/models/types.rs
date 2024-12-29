use std::fmt;
use serde::Serialize;

#[derive(Serialize)]
pub enum MarketType {
    Stocks,
    Forex,
    Crypto
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stocks => write!(f, "stocks"),
            Self::Forex  => write!(f, "forex"),
            Self::Crypto => write!(f, "crypto"),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Gainers,
    Losers
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gainers => write!(f, "gainers"),
            Self::Losers  => write!(f, "losers")
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetClass {
    Stocks,
    Options,
    Crypto,
    Fx,
    OTC,
    Indicies
}


#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Trade,
    BBO,
    NBBO
}

#[derive(Serialize)]
pub enum SIP {
    CTA,
    UTP,
    OPRA
}

#[derive(Serialize)]
pub enum Frequency {
    #[serde(rename = "0")]
    OneTime,
    #[serde(rename = "1")]
    Annually,
    #[serde(rename = "2")]
    BiAnnually,
    #[serde(rename = "4")]
    Quarterly,
    #[serde(rename = "12")]
    Monthly
}

#[derive(Serialize)]
pub enum DividendType {
    CD, // CD Dividend (Normal)
    LT, // Long Term Dividend
    SC, // Special Cash Dividend
    ST  // Short Term Dividend
}

#[derive(Serialize)]
pub enum Comparator {
    EQ,
    LT,
    LTE,
    GT,
    GTE
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Timeframe {
    Annual,
    Quarterly,
    TrailingTwelveMonths
}

#[derive(Serialize)]
pub enum MarketLocale {
    US,
    Global
}

impl fmt::Display for MarketLocale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::US => write!(f, "us"),
            Self::Global => write!(f, "global")
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Timespan {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Second  => write!(f, "second"),
            Self::Minute  => write!(f, "minute"),
            Self::Hour    => write!(f, "hour"),
            Self::Day     => write!(f, "day"),
            Self::Week    => write!(f, "week"),
            Self::Month   => write!(f, "month"),
            Self::Quarter => write!(f, "quarter"),
            Self::Year    => write!(f, "year"),
        }
    }
}

#[derive(Serialize)]
pub enum Sort {
    FilingDate,
    PeriodOfReportDate
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SeriesType {
    High,
    Open,
    Low,
    Close
}
