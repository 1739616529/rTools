
use anyhow::{Error, Result};

/// 匹配 event
///
/// # Examples
///
/// 正常情况
/// ```
/// let event = event_match("core:hotkey:setup:open.main.window")
/// assert_eq!(event, Ok(("core", "hotkey", "setup", "open.main.window")));
/// ```
///
/// 未传参数
/// ```
/// let event = event_match("")
/// assert_eq!(event, Err("parse event  error"));
/// ```
///
/// 参数不是 event
/// ```
/// let event = event_match("abc")
/// assert_eq!(event, Err("parse event  error"));
/// ```

pub fn event_match(event: &str) -> Result<(&str, &str, &str, &str)> {
    let event_vec: Vec<&str> = event.split(":").collect();

    // 如果 不是 正常 event
    if event_vec.len() != 4 {
        return Err(Error::msg(format!("parse event {event} error")))
    }
    Ok((event_vec[0], event_vec[1], event_vec[2], event_vec[2]))
}
