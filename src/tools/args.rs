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

    #[arg(short = 'n', long = "process-name", help = "Filter by process name (partial match)")]
    pub process_name: Option<String>,

    #[arg(short = 'i', long, help = "Filter by process ID (PID)")]
    pub pid: Option<u32>,

    #[arg(short = 's', long, help = "Filter by connection state (e.g., LISTEN, ESTABLISHED)")]
    pub state: Option<String>,

    #[arg(short = 'v', long = "ip-version", help = "Specify IP version (4 for IPv4, 6 for IPv6). Defaults to IPv4.")]
    pub ip_version: Option<u8>,

    #[arg(long = "local-ip", help = "Filter by local IP address")]
    pub local_ip: Option<String>,

    #[arg(long = "remote-ip", help = "Filter by remote IP address")]
    pub remote_ip: Option<String>,

    #[arg(long = "json", help = "Output results in JSON format")]
    pub json: bool,

    #[arg(long = "no-system", help = "Exclude system processes (e.g., PID 4) from the output")]
    pub no_system: bool,

    #[arg(long = "sort-asc", help = "Sort output ascending by specified field: pid, name, port, remote port (rport) or uptime")]
    pub sort_asc_by: Option<String>,

    #[arg(long = "sort-desc", help = "Sort output descending by specified field: pid, name, port, remote port (rport) or uptime")]
    pub sort_desc_by: Option<String>,

    #[arg(long = "resolve-hostname", help = "Tries to replace remote IP address with resolved hostname through DNS lookup if possible (can take few seconds)")]
    pub resolve_hostname: bool,

    #[arg (long = "compact", help = "Removes table borders from output")]
    pub compact: bool
}

impl Args {
    pub fn get_argc(&self) -> usize {
        self.port.is_some() as usize + 
        self.remote_port.is_some() as usize +
        self.process_name.is_some() as usize +
        self.pid.is_some() as usize +
        self.state.is_some() as usize +
        self.local_ip.is_some() as usize +
        self.remote_ip.is_some() as usize +
        self.no_system as usize
    }
}