use chrono::{Datelike, Local, NaiveDate, Duration};

fn main() {
    // 抓取當前的日期
    let today = Local::now();
    let last_day = last_day_of_month(today.year(), today.month());
    // 印出今天的日期
    println!("Today's date is: {} {}", today.format("%Y-%m-%d"), last_day.day);
}


fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    // 取得該月的第一天
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    // 取得下一個月的第一天，然後往前一天就是本月的最後一天
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    let last_day = next_month.unwrap() - Duration::days(1);

    last_day
}