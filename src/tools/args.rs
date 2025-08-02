use clap::{arg, Parser};
use colored::Colorize;
use crate::tools::print_utils::OUTPUT_FIELDS;

#[derive(Parser, Debug)]
#[command(name = "r-port-doctor", version, about = "Port debug and diagnostic tool")]
pub struct Args {
    #[arg(short = 'l', long, help = "Filter by local port number")]
    pub port: Option<u16>,

    #[arg(short = 'r', long = "remote-port", help = "Filter by remote port number")]
    pub remote_port: Option<u16>,

    #[arg(short = 'm', long, help = "Filter by protocol (TCP or UDP)")]
    pub mode: Option<String>,

    #[arg(short = 'n', long = "process-name", help = "Filter by process name (regular expression)")]
    pub process_name: Option<String>,

    #[arg(short = 'i', long, help = "Filter by process ID (PID)")]
    pub pid: Option<u32>,

    #[arg(short = 's', long, help = "Filter by connection state (e.g., LISTEN, ESTABLISHED)")]
    pub state: Option<String>,

    #[arg(short = 'v', long = "ip-version", help = "Specify IP version 4 (IPv4) or 6 (IPv6). Defaults to IPv4.")]
    pub ip_version: Option<u8>,

    #[arg(long = "local-address", help = "Filter by local IP address")]
    pub local_address: Option<String>,

    #[arg(long = "remote-address", help = "Filter by remote IP address")]
    pub remote_address: Option<String>,

    #[arg(long = "json", help = "Output results in JSON format")]
    pub json: bool,

    #[arg(long = "no-system", help = "Exclude system processes (e.g., PID 4) from the output")]
    pub no_system: bool,

    #[arg(long = "sort", help = "Sort output in the specified order (asc/desc) by the specified field 
Available fields:
    - pid
    - port
    - remote-port
    - process-name
    - uptime
    - local-address
    - remote-address",
    value_names = ["ORDER", "FIELD"], num_args = 2)]
    pub sort_by: Option<Vec<String>>,

    #[arg(long = "resolve-hostname", help = "Resolve remote IP addresses to hostnames using DNS (may take a few seconds for IPv4 addresses)")]
    pub resolve_hostname: bool,

    #[arg(long = "compact", help = "Removes table borders from output")]
    pub compact: bool,

    #[arg(long = "older-than", help = "Filter connections by uptime being older than provided seconds", value_name = "SECONDS")]
    pub older_than: Option<u32>,

    #[arg(long = "younger-than", help = "Filter connections by uptime being younger than provided seconds", value_name = "SECONDS")]
    pub younger_than: Option<u32>,

    #[arg(long = "fields", help = "Show only the specified fields in the table (all shown by default)", value_name = "FIELD", num_args = 1..=8)]
    pub fields: Option<Vec<String>>,

    #[arg(long = "uptime", help = "Specify uptime format:
    - clock (HH:MM:SS)
    - human (DDd HHh MMm SSs)
    - hours
    - minutes
    - seconds")]
    pub uptime_format: Option<String>,

    #[arg(long = "range", help = "Filter rows by value ranges of the specified field
Available fields:
    - pid
    - port  
    - remote-port
    - uptime (uses seconds)
    - local-address
    - remote-address",
    value_names = ["FIELD", "MIN", "MAX"], num_args = 3)]
    pub range: Option<Vec<String>>,

    #[arg(long = "stats", help = "Outputs various statistics related to the socket table instead of the table itself")]
    pub stats: bool,

    #[arg(long = "set", help = "Set or reset a default value in the configuration file.
To set a value: --set <KEY> <VALUE> (e.g., --set port 8080)
To reset a value: --set <KEY> (e.g., --set port)
Available fields:
    - port
    - remote_port
    - mode
    - process_name
    - pid
    - state
    - local_address
    - remote_address
    - uptime_format", 
    value_names = ["KEY", "VALUE"], num_args = 1..=2)]
    pub set_config_value: Option<Vec<String>>,

    #[arg(long = "get", help = "Get the default value of the specified field from the configuration file
To get a value: --get <KEY> (e.g, --get port)",
    value_name = "KEY")]
    pub get_config_value: Option<String>
}

impl Args {
    pub fn get_argc(&self) -> usize {
        self.port.is_some() as usize + 
        self.remote_port.is_some() as usize +
        self.process_name.is_some() as usize +
        self.pid.is_some() as usize +
        self.state.is_some() as usize +
        self.local_address.is_some() as usize +
        self.remote_address.is_some() as usize +
        self.no_system as usize +
        self.older_than.is_some() as usize +
        self.younger_than.is_some() as usize +
        self.range.is_some() as usize
    }
}

pub fn validate_field_args(fields: Option<&Vec<String>>) {
    if let Some(fields) = fields {
        let mut seen_fields = std::collections::HashSet::new();
        for field in fields {
            let lower_field = field.to_lowercase();
            if !seen_fields.insert(lower_field.clone()) {
                eprintln!("error: Repeated field '{}'", field.bold().underline());
                std::process::exit(0);
            }
            if !OUTPUT_FIELDS.contains(&lower_field.as_str()) {
                eprintln!("error: Invalid field: '{}'\n\nAvailable fields:\n\n  - pid\n  - process_name\n  - port\n  - protocol\n  - local_address\n  - remote_address\n  - state\n  - uptime", field.bold().underline());
                std::process::exit(0);
            }
        }
    }
}