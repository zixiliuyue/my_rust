//! Day 12：CLI 参数和配置默认值。

use crate::course_utils::print_banner;

pub fn run_with_args(args: Vec<String>) -> Result<(), String> {
    print_banner(12, "CLI 参数和配置默认值");

    let config = CliConfig::from_args(&args)?;
    println!("service={} dry_run={}", config.service, config.dry_run);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct CliConfig {
    service: String,
    dry_run: bool,
}

impl CliConfig {
    fn from_args(args: &[String]) -> Result<Self, String> {
        let mut service = String::from("demo-service");
        let mut dry_run = false;
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--service" => {
                    index += 1;
                    service = args
                        .get(index)
                        .ok_or_else(|| String::from("--service 后面需要服务名"))?
                        .clone();
                }
                "--dry-run" => dry_run = true,
                other => return Err(format!("未知参数：{other}")),
            }

            index += 1;
        }

        Ok(Self { service, dry_run })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_config() {
        let args = vec![
            String::from("--service"),
            String::from("api"),
            String::from("--dry-run"),
        ];

        assert_eq!(
            CliConfig::from_args(&args).expect("参数应该合法"),
            CliConfig {
                service: String::from("api"),
                dry_run: true
            }
        );
    }
}
