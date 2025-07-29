use serde::Deserialize;
use std::fs;
use std::io::Write;
use crate::tools::args::Args;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub port: Option<u16>,
    pub remote_port: Option<u16>,
    pub mode: Option<String>,
    pub process_name: Option<String>,
    pub pid: Option<u32>,
    pub state: Option<String>,
    pub ip_version: Option<u8>,
    pub local_address: Option<String>,
    pub remote_address: Option<String>,
}

fn get_default_config_content() -> String {
    r#"
# Default configuration for r-port-doctor
# Uncomment and set the values you want to use as default.

# port = 80
# remote_port = 443
# mode = "TCP"
# process_name = "svchost.exe"
# pid = 4
# state = "ESTABLISHED"
# ip_version = 4
# local_address = "127.0.0.1"
# remote_address = "0.0.0.0"
    "#.to_string()
}

pub fn get_config() -> Option<Config> {
    let config_dir = dirs::config_dir()?;
    let config_dir_path = config_dir.join("r-port-doctor");
    let config_file_path = config_dir_path.join("config.toml");
    
    if !config_file_path.exists() {
        if fs::create_dir_all(&config_dir_path).is_ok() {
            if let Ok(mut file) = fs::File::create(&config_file_path) {
                let _ = file.write_all(get_default_config_content().as_bytes());
            }
        }
    }

    let content = fs::read_to_string(config_file_path).ok()?;
    let config = toml::from_str(&content).ok();
    config
}

pub fn apply_config(config: Config, args: &mut Args) {
    args.port = args.port.or(config.port);
    args.remote_port = args.remote_port.or(config.remote_port);
    args.mode = args.mode.clone().or(config.mode);
    args.process_name = args.process_name.clone().or(config.process_name);
    args.pid = args.pid.or(config.pid);
    args.state = args.state.clone().or(config.state);
    args.ip_version = args.ip_version.or(config.ip_version);
    args.local_address = args.local_address.clone().or(config.local_address);
    args.remote_address = args.remote_address.clone().or(config.remote_address);
}