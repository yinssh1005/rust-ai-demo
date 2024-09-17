// mod dbConnect;

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Offset, ParseResult, Utc};
use chrono_tz::ParseError;
use clap::builder::Str;
use clap::{Arg, Command};
use dialoguer::Input;
use log::debug;
use std::num::ParseIntError;
use std::str::FromStr;

fn is_valid_format(time_str: &str, format: &str) -> bool {
    NaiveDateTime::parse_from_str(time_str, format).is_ok()
}

fn get_system_timezone_as_fixedoffset() -> FixedOffset {
    // 获取当前系统本地时间
    let local_now = Local::now();

    // 从本地时间获取时区偏移量
    let offset_from_utc = local_now.offset().fix();

    // 返回时区偏移量
    offset_from_utc
}
fn calculate_fixed_offset_difference(offsetTz: FixedOffset) -> FixedOffset {
    let system_offset = get_system_timezone_as_fixedoffset();

    // 直接计算两个时区偏移量之间的差异
    // let hours_diff = system_offset.local_minus_utc() as i32 - offsetTz.local_minus_utc() as i32;
    let hours_diff = offsetTz.local_minus_utc() as i32 - system_offset.local_minus_utc() as i32;
    parse_offset(hours_diff).unwrap()
}

fn convert_to_utc(time_str: &str, format_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_date_time = NaiveDateTime::parse_from_str(time_str, format_str)?;
    // 将无时区的 NaiveDateTime 转换为 UTC 时区的 DateTime
    Ok(DateTime::from_naive_utc_and_offset(naive_date_time, Utc))
}

fn parse_offset(hours: i32) -> Result<FixedOffset, String> {
    FixedOffset::east_opt(hours)
        .ok_or_else(|| format!("Invalid timezone offset: {}", hours))
}

fn parse_offset_str(timezone: &str) -> Result<FixedOffset, String> {
    let hours_str = &timezone[3..];
    let sign = if hours_str.starts_with('+') { 1 } else { -1 };

    match timezone[3..].parse::<i32>() {
        Ok(hours) => {
            FixedOffset::east_opt(sign * 3600 * hours)
                .ok_or_else(|| format!("Invalid timezone offset: {}", timezone))
        }
        Err(e) => Err(e.to_string()), // Directly forward the ParseIntError as a String
    }
}

/// 转换 meeting_time 从 local_timezone 到 target_timezone
///
/// # 参数
/// * `meeting_time` - 会议日期时间（字符串形式，例如："2023-04-05 14:30"）
/// * `target_timezone` - 目标时区标识符（例如："UTC-05:00"）
///
/// # 返回值
/// 转换后的日期时间字符串
fn convert_meeting_timezone(
    meeting_time: &str,
    target_timezone: &str,
) -> DateTime<FixedOffset> {
    env_logger::init();

    // 解析会议日期时间字符串到 naive datetime，并假设为本地时间
    let naive_meeting_time = convert_to_utc(meeting_time, "%Y-%m-%d %H:%M");

    debug!("Debug -- naive_meeting_time is {:?}", naive_meeting_time);

    // Extract hours part and parse it, handling the error explicitly
    let target_offset = calculate_fixed_offset_difference(parse_offset_str(target_timezone).unwrap());
    debug!("Debug -- target_offset is {}", target_offset);

    // 将本地时间转换为 UTC
    let utc_datetime = naive_meeting_time.unwrap().with_timezone(&Utc);
    debug!("Debug -- utc_datetime is {}", utc_datetime);

    let rst_dt: DateTime<FixedOffset> = utc_datetime.with_timezone(&target_offset);

    rst_dt
}

fn main() {

    // 日期格式要求 YYYY-MM-DD HH:MM
    let format = "%Y-%m-%d %H:%M";

    let matches = Command::new("时区转换工具CLI")
        .version("1.0")
        .author("Ethan.Yin <ethan.yin@jetbrains.com>")
        .about("A simple interactive CLI tool example")
        .arg(
            Arg::new("option")
                .help("-h")
        )
        .get_matches();

    let meetingDateTime: String = Input::new()
        .with_prompt("请输入你时区的会议时间，以 'YYYY-MM-DD HH:MM' 的格式")
        .interact_text()
        .unwrap();

    // 检查输入的日期是否正确
    match is_valid_format(meetingDateTime.as_str(), format) {
        true => {}

        false => {
            panic!("日期格式不正确：{} -> YYYY-MM-DD HH:MM", meetingDateTime);
        }
    }

    // 输入目标时区
    let targetTimezone: String = Input::new()
        .with_prompt("转换目标时区(UTC)?(UTC+2)")
        .default("UTC+2".to_string())
        .interact_text()
        .unwrap();

    println!("您输入的转换目标时区: {}", targetTimezone);

    let rst_dt: DateTime<FixedOffset> = convert_meeting_timezone(&meetingDateTime, &targetTimezone);

    println!("转换后的会议时间: {}", rst_dt);
}