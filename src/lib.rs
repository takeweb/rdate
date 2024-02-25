use chrono::{Datelike, Local, Timelike, Utc, Weekday};
use std::time::SystemTime;

pub fn run() {
    let local_datetime = Local::now();
    let year = local_datetime.year();
    let month = local_datetime.month();
    let day = local_datetime.day();
    let weekday = format_japan_weekday(&local_datetime.weekday());
    let hour = local_datetime.hour();
    let minute = local_datetime.minute();
    let second = local_datetime.second();

    // 無加工の値
    println!("{}", local_datetime.naive_local());

    // 独自フォーマット
    // dateコマンドの「2022年 12月16日 金曜日 08時50分14秒 JST」を目指す
    let timezone = local_datetime.timezone();
    // let timezone = local_datetime.naive_local();
    println!(
        "{:4}年{:02}月{:02}日 {}曜日 {:02}時{:02}分{:02}秒 {:?}",
        year, month, day, weekday, hour, minute, second, timezone
    );
    println!(
        "{:4}年{:02}月{:02}日 {}曜日 {:02}時{:02}分{:02}秒 JST",
        year, month, day, weekday, hour, minute, second
    );

    // JST
    println!(
        "{}",
        local_datetime
            .format("%Y年%m月%d日 %H時%M分%S秒 %Z")
            .to_string()
    );

    // UTC
    let utc = Utc::now();
    let text = utc.format("%Y年%m月%d日 %H時%M分%S秒 %Z").to_string();
    println!("{}", text);

    // i64型で帰ってくる。
    let unix_sec = utc.timestamp() as f64;
    let unix_nanosec = utc.timestamp_nanos_opt().unwrap() as f64;

    // nano秒までを f64型で表現する
    let sec = unix_sec + unix_nanosec / 1e9;
    println!("UNIX秒:{}", sec);

    // stdライブラリでシステム日付
    let sys_time = SystemTime::now();
    println!("{:?}", sys_time);
}

fn format_japan_weekday(weekday: &Weekday) -> String {
    match weekday {
        Weekday::Mon => String::from("月"),
        Weekday::Tue => String::from("火"),
        Weekday::Wed => String::from("水"),
        Weekday::Thu => String::from("木"),
        Weekday::Fri => String::from("金"),
        Weekday::Sat => String::from("土"),
        Weekday::Sun => String::from("日"),
    }
}
