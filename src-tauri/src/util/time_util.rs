use std::fmt::Debug;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, TimeZone, Utc};

pub fn get_second_timestamp_by_str(time_str: &str) -> i64 {
    let date = NaiveDate::parse_from_str(time_str, "%Y-%m-%d").unwrap();
    let option = date.and_hms_opt(0, 0, 0).unwrap();
    let option1 = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    let i2 = option.and_local_timezone(option1).unwrap().timestamp();
    i2
}

pub fn get_today_time_stamp() -> i64 {
    let time = Local::now();
    let noon = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    time.with_time(noon).unwrap().timestamp()
}

pub fn get_date_from_time_stamp(time_stamp: i64) -> String {
    let time = DateTime::from_timestamp(time_stamp , 0).expect(format!("timestamp transport error :{}", time_stamp).as_str());
    // 将时间戳转换为 NaiveDateTime
    use chrono_tz::Asia::Chongqing;
    let time1 = time.naive_local();
    let datetime_chongqing = Chongqing.from_utc_datetime(&time1);
    // 打印日期
    let formatted_datetime = datetime_chongqing.format("%Y-%m-%d").to_string();
    formatted_datetime
}

#[cfg(test)]
mod test {
    use chrono::{FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
    use chrono::format::{parse, Parsed, StrftimeItems};

    #[test]
    fn test_blank_condition() {
        let result = NaiveDate::parse_from_str("2024-09-14", "%Y-%m-%d").unwrap();
        let option = result.and_hms_opt(21, 15, 0).unwrap();

        let option1 = FixedOffset::east_opt(8 * 60 * 60).unwrap();
        // 仅能查询出当UTC 时间
        let i2 = option.and_local_timezone(option1).unwrap().timestamp();
        let i = Local::now().timestamp();
        println!("{}", i - i2);
    }

    #[test]
    fn test_now_date_time_str() {
        let now = Local::now();
        let formatted_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("格式化后的当前日期和时间: {}", formatted_date);
    }


    #[test]
    fn test_today_time_stamp() {
        let time = Local::now();
        let noon = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let i2 = time.with_time(noon).unwrap().timestamp();
        let i = Local::now().timestamp();
        println!("{}", i - i2);
    }

}