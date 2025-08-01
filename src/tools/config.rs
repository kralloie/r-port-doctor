use colored::Colorize;
use regex::Regex;
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
    pub uptime_format: Option<String>
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

# uptime_format = "human"
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

pub fn apply_config(config: Option<Config>, args: &mut Args) {
    if let Some(config) = config {
        args.port = args.port.or(config.port);
        args.remote_port = args.remote_port.or(config.remote_port);
        args.mode = args.mode.clone().or(config.mode);
        args.process_name = args.process_name.clone().or(config.process_name);
        args.pid = args.pid.or(config.pid);
        args.state = args.state.clone().or(config.state);
        args.ip_version = args.ip_version.or(config.ip_version);
        args.local_address = args.local_address.clone().or(config.local_address);
        args.remote_address = args.remote_address.clone().or(config.remote_address);
        args.uptime_format = args.uptime_format.clone().or(config.uptime_format)
    }
}

// true = store as string | false = store as integer
const CONFIG_KEYS: [(&'static str, bool); 10] =  [
    ("port", false),
    ("remote_port", false),
    ("mode", true),
    ("process_name", true),
    ("pid", false),
    ("state", true),
    ("ip_version", false),
    ("local_address", true),
    ("remote_address", true),
    ("uptime_format", true)
];

fn verify_config_file() {
    let config_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => {
            eprintln!("error: Config directory not found");
            std::process::exit(0);
        }
    };
    let config_dir_path = config_dir.join("r-port-doctor");
    let config_file_path = config_dir_path.join("config.toml");

    if !config_file_path.exists() {
        if fs::create_dir_all(&config_dir_path).is_ok() {
            if let Ok(mut file) = fs::File::create(&config_file_path) {
                let _ = file.write_all(get_default_config_content().as_bytes());
            }
        }
    }
}

pub fn get_config_value(key: &Option<String>) {
    if let Some(k) = key {
        let config = get_config();
        if let Some(conf) = config {
            let value: Option<String> = match k.to_lowercase().as_str() {
                "port" => conf.port.map(|v| v.to_string()),
                "remote_port" =>  conf.remote_port.map(|v| v.to_string()),
                "mode" => conf.mode,
                "process_name" => conf.process_name,
                "pid" => conf.pid.map(|v| v.to_string()),
                "state" => conf.state,
                "ip_version" => conf.ip_version.map(|v| v.to_string()),
                "local_address" => conf.local_address,
                "remote_address" => conf.remote_address,
                "uptime_fromat" => conf.uptime_format,
                _ => None
            };
            if let Some(v) = value {
                println!("{}", v.bold().underline());
            }
        }
        std::process::exit(0);
    }
}

pub fn set_config_value(key: &str, value: Option<&String>) -> std::io::Result<()> {
    verify_config_file();
    let config_file_path = dirs::config_dir()
        .unwrap()
        .join("r-port-doctor")
        .join("config.toml");
    let config_file_content = fs::read_to_string(&config_file_path)?;
    let mut config_file_lines: Vec<String> = config_file_content.lines().map(String::from).collect();
    let target_key_idx = CONFIG_KEYS.iter().position(|k| k.0 == key).unwrap_or_else(|| {
        eprintln!("error: Invalid configuration key: '{}'\n\nUse '--help' to see available configurations or read the configuration file on 'AppData\\Roaming\\r-port-doctor\\config.toml'", key);
        std::process::exit(0);
    });
    let regex = Regex::new(format!(r"^#?\s*{}\s*=.*", CONFIG_KEYS[target_key_idx].0).as_str()).unwrap();
    let mut found_line = false;
    config_file_lines.iter_mut().for_each(|line| {
        if regex.is_match(line.trim()) {
            found_line = true;
            if value.is_some() {
                if CONFIG_KEYS[target_key_idx].1 {
                    *line = format!("{} = \"{}\"", key, value.unwrap());
                } else {
                    *line = format!("{} = {}", key, value.unwrap());
                }
            } else {
                *line = format!("# {} =", key);
            }
        }
    });

    if !found_line {
        if value.is_some() {
            if CONFIG_KEYS[target_key_idx].1 {
                config_file_lines.push(format!("{} = \"{}\"", key, value.unwrap()));
            } else {
                config_file_lines.push(format!("{} = {}", key, value.unwrap()));
            }
        } else {
            config_file_lines.push(format!("# {} =", key));
        }
    }

    let new_file_content = config_file_lines.join("\n");
    fs::write(config_file_path, new_file_content)
}

pub fn update_config(config_value: Option<Vec<String>>) {
    if let Some(value) = config_value {
        let update_config = set_config_value(value[0].as_str(), value.get(1));
        match update_config {
            Ok(()) => println!("Updated configuration: {} set to {}", value[0].bold().underline(), value.get(1).unwrap_or(&String::from("none")).bold().underline()),
            Err(e) => eprintln!("error: {}", e)
        }
        std::process::exit(0)
    }
}