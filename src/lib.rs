pub mod command;
pub mod config;
pub mod tools;
pub mod utils;
pub mod venus;

use command::Cmd;
use config::set_config_token;

pub use crate::config::parse_config;

pub fn run(cmd_str: &str) -> Result<(), String> {
    let cmd = Cmd::from_str(cmd_str)
        .ok_or(format!("cant support cmd {}", cmd_str))?;

    parse_config()
        .map_err(|e| format!("parse config err: {}", e.to_string()))?;

    let token = venus::get_token()
        .or_else(|e| Err(format!("parse config err: {}", e)))?;
    set_config_token(String::from(&token));

    match cmd {
        Cmd::Deploy => {
            let user_info = venus::get_user_info(String::from(&token))
                .map_err(|e| format!("get_user_info err: {}", e))?;
            command::deploy(&user_info)
                .map_err(|e| format!("deploy tier err: {}", e))
        }
        Cmd::Copy => {
            command::copy().map_err(|e| format!("copy tier err: {}", e))
        }
        Cmd::Open => {
            command::open().map_err(|e| format!("open tier err: {}", e))
        }
    }
}
