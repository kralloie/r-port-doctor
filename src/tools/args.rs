use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(name = "r-port-doctor", version, about = "Port debug and diagnostic tool", ignore_errors = true)]
pub struct Args {
    #[arg(short = 'p', long, help = "Filter by local port")]
    pub port: Option<u16>,

    #[arg(short = 'm', long, help = "Filter by protocol (UDP/TCP)")]
    pub mode: Option<String>,

    #[arg(short = 'n', long = "process-name", help = "Filter by similar process name")]
    pub process_name: Option<String>,

    #[arg(short = 'i', long, help = "Filter by PID")]
    pub pid: Option<u32>,

    #[arg(short = 's', long, help = "Filter by socket state (LISTEN, ESTABLISHED, etc)")]
    pub state: Option<String>
}

impl Args {
    pub fn get_argc(&self) -> usize {
        self.port.is_some() as usize + 
        self.mode.is_some() as usize + 
        self.process_name.is_some() as usize +
        self.pid.is_some() as usize +
        self.state.is_some() as usize
    }
}