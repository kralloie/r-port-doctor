use clap::{arg, Parser};

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

    #[arg(long = "sort", help = "Sort output in the specified order by the specified field\nAvailable fields:\n  - pid\n  - port\n  - remote-port\n  - process-name\n  - uptime\n  - local-address\n  - remote-address", value_names = ["ORDER", "FIELD"], num_args = 2)]
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

    #[arg(long = "uptime", help = "Specify uptime format:\n  - clock (HH:MM:SS)\n  - human (DDd HHh MMm SSs)\n  - hours\n  - minutes\n  - seconds")]
    pub uptime_format: Option<String>,

    #[arg(long = "range", help = "Filter rows by value ranges of the specified field\nAvailable fields:\n  - pid\n  - port\n  - remote-port\n  - uptime (uses seconds)\n  - local-address\n  - remote-address", value_names = ["FIELD", "MIN", "MAX"], num_args = 2..=3)]
    pub range: Option<Vec<String>>,

    #[arg(long = "stats", help = "Outputs various statistics related to the socket table instead of the table itself")]
    pub stats: bool,

    #[arg(long = "set", help = "Set configuration file values, leave empty for resetting field value, e.g '--set port'", value_names = ["KEY", "VALUE"], num_args = 1..=2)]
    pub set_config_value: Option<Vec<String>>
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