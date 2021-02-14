use chrono::{Datelike, Weekday};

pub (crate) mod stock_actor;
pub (crate) mod options_actor;
pub (crate) mod push_notifications_actor;

pub fn is_open_market_hours(dt: chrono::DateTime<chrono::Utc>) -> bool {
    let is_weekday = match dt.weekday() {
        Weekday::Sat | Weekday::Sun => false,
        _ => true
    };

    let mkt_open = chrono::NaiveTime::from_hms(14, 30, 0); //2:30pm UTC == 9:30am EST (UTC-5)
    let mkt_close = chrono::NaiveTime::from_hms(21, 00, 0); //9:00pm UTC == 4:00pm EST
    return is_weekday && dt.time() >= mkt_open && dt.time() <= mkt_close;
}

#[test]
fn open_market_hours() {
    use chrono::{DateTime, TimeZone};
    //using local PST where market is open 06:30 to 13:00
    assert!(is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 8).and_hms(6, 30, 0)))); //friday at open
    assert!(is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 8).and_hms(13, 00, 0)))); //friday at close
    assert!(is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 4).and_hms(10, 00, 0)))); //monday midday
    assert!(!is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 4).and_hms(15, 00, 0)))); //monday after hours
    assert!(!is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 8).and_hms(13, 01, 0)))); //friday after close
    assert!(!is_open_market_hours(DateTime::from(chrono::Local.ymd(2021, 1, 9).and_hms(6, 30, 0)))); //saturday
}


