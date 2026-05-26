//! Day 9：模块边界和自定义错误。

use crate::course_utils::print_banner;
use std::fmt;

pub fn run() {
    print_banner(9, "模块边界和自定义错误");

    let config = Config::parse("service=checkout-api\nretries=3");
    match config {
        Ok(config) => println!(
            "配置解析成功：{} retries={}",
            config.service, config.retries
        ),
        Err(error) => println!("配置解析失败：{error}"),
    }
}

/// 课程模块里的公开模型。字段保持 `pub`，因为 binary 或后续测试可能需要读取。
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub service: String,
    pub retries: u8,
}

impl Config {
    pub fn parse(input: &str) -> Result<Self, ConfigError> {
        let mut service = None;
        let mut retries = None;

        for line in input.lines() {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| ConfigError::InvalidLine(line.to_string()))?;

            match key.trim() {
                "service" => service = Some(value.trim().to_string()),
                "retries" => {
                    retries = Some(
                        value
                            .trim()
                            .parse::<u8>()
                            .map_err(|_| ConfigError::InvalidRetries(value.to_string()))?,
                    );
                }
                other => return Err(ConfigError::UnknownKey(other.to_string())),
            }
        }

        Ok(Self {
            service: service.ok_or(ConfigError::MissingField("service"))?,
            retries: retries.unwrap_or(1),
        })
    }
}

/// 自定义错误枚举让失败原因可枚举、可测试、可展示。
#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    InvalidLine(String),
    InvalidRetries(String),
    MissingField(&'static str),
    UnknownKey(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidLine(line) => write!(f, "配置行缺少 '='：{line}"),
            ConfigError::InvalidRetries(value) => write!(f, "retries 不是 u8：{value}"),
            ConfigError::MissingField(field) => write!(f, "缺少必填字段：{field}"),
            ConfigError::UnknownKey(key) => write!(f, "未知配置项：{key}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_successfully() {
        let config = Config::parse("service=api\nretries=2").expect("配置应该能解析");

        assert_eq!(
            config,
            Config {
                service: String::from("api"),
                retries: 2
            }
        );
    }

    #[test]
    fn missing_service_is_error() {
        assert_eq!(
            Config::parse("retries=2").unwrap_err(),
            ConfigError::MissingField("service")
        );
    }
}
