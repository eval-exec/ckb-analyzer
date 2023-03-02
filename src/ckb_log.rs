#![warn(dead_code)]
use log::info;

#[derive(Debug)]
pub struct CKBLog {
    pub timestamp: chrono::NaiveDateTime,
    pub runtime_name: String,
    pub log_level: String,
    pub service_module: String,
    pub log_content: String,
}
/*
2023-02-28 08:58:24.927 +00:00 ChainService INFO ckb_chain::chain  analyze: non-contextual verify done: 7302133, elapsed: 23109
 */
pub(crate) fn parse<F>(line: &str, filter: F) -> Option<CKBLog>
where
    F: Fn(&str) -> bool,
{
    // trim start and end whitespace of line
    let line = line.trim();
    if !filter(line) {
        return None;
    }

    let time_zone_i = line.find(" +00:00 ")?;
    if time_zone_i < 23 {
        info!("time_zone_i: {} < 23: {}", time_zone_i, line);
        return None;
    }

    let time_str = line[time_zone_i - 23..time_zone_i].to_string();
    let timestamp = chrono::NaiveDateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S%.f")
        .unwrap_or_else(|v| {
            panic!("parse time error: {}, text: {}", v, time_str);
        });
    let line = line[time_zone_i + 7..].to_string();
    let runtime_name_end = line.find(" ")?;
    let runtime_name = line[..runtime_name_end].to_string();

    let line = line[runtime_name_end + 1..].to_string();
    let log_level_end = line.find(" ")?;
    let log_level = line[..log_level_end].to_string();

    let line = line[log_level_end + 1..].to_string();
    let service_module_end = line.find(" ")?;
    let service_module = line[..service_module_end].to_string();
    let log_content = line[service_module_end + 1..].to_string();

    Some(CKBLog {
        timestamp,
        runtime_name,
        log_level,
        service_module,
        log_content,
    })
}
