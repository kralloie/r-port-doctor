use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(name = "r-port-doctor", version, about = "Port debug and diagnostic tool")]
pub struct Args {
    #[arg(short = 'p', long)]
    pub port: Option<u16>,

    #[arg(short = 'm', long)]
    pub mode: Option<String>,

    #[arg(short = 'n', long)]
    pub processname: Option<String>,

    #[arg(short = 'i', long)]
    pub pid: Option<u32>,

    #[arg(short = 's', long)]
    pub state: Option<String>,
}

impl Args {
    pub fn get_argc(&self) -> usize {
        self.port.is_some() as usize + 
        self.mode.is_some() as usize + 
        self.processname.is_some() as usize +
        self.pid.is_some() as usize +
        self.state.is_some() as usize
    }
}