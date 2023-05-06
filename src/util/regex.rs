use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DOUBLE_WHITE_SPACE_RE: Regex = Regex::new(r"\s\s+").unwrap();
    static ref SPECIAL_CHAR: Regex = Regex::new(r"@#`:\r\n\t\f\v\p{C}").unwrap();
    static ref CHANNEL_MENTION: Regex = Regex::new(r"<#(\d+)>").unwrap();
    static ref USER_MENTION: Regex = Regex::new(r"<@!?(\d+)>").unwrap();
    static ref ROLE_MENTION: Regex = Regex::new(r"<@&(\d+)>").unwrap();
    static ref EVERYONE_MENTION: Regex = Regex::new(r"@everyone").unwrap();
    static ref HERE_MENTION: Regex = Regex::new(r"@here").unwrap();
}
