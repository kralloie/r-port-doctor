use std::fmt::{Display, Formatter};
use windows::Win32::
        Networking::WinSock::{AF_INET, AF_INET6}
    ;
use colored::*;
use crate::tools::{args::Args, print::*};
use serde::Serialize;

//--------------------------------------------------------------------------------------------------------------------------
#[derive(Clone, Serialize)]
pub struct Socket {
    pub process_name: String,
    pub pid: u32,
    pub port: u16,
    pub protocol: &'static str,
    pub local_addr: String,
    pub remote_addr: Option<String>,
    pub remote_port: Option<u16>,
    pub state: String,
    pub executable_path: Option<String>
}

pub const IPV4_ULAF: u32 = AF_INET.0 as u32;
pub const IPV6_ULAF: u32 = AF_INET6.0 as u32;

fn filter_socket_table (args: &Args, argc: usize, socket: &&Socket) -> bool {
    let mut filter_count = 0;
    if let Some(p) = args.port {
        filter_count = filter_count + (socket.port == p) as usize;
    }

    if let Some(m) = args.mode.clone() {
        filter_count = filter_count + (socket.protocol.to_lowercase() == m.to_lowercase()) as usize;
    }

    if let Some(n) = args.process_name.clone() {
        filter_count = filter_count + (socket.process_name.to_lowercase().contains(n.to_lowercase().as_str())) as usize;
    }

    if let Some(i) = args.pid {
        filter_count = filter_count + (socket.pid == i) as usize;
    }

    if let Some(s) = args.state.clone() {
        filter_count = filter_count + (socket.state.to_string().to_lowercase() == s.to_lowercase()) as usize;
    }

    if let Some(l) = args.local_ip.clone() {
        filter_count = filter_count + (socket.local_addr.to_string().to_lowercase() == l.to_lowercase()) as usize;
    }

    if let Some(r) = args.remote_ip.clone() {
        if let Some(remote_addr) = socket.remote_addr.clone() {
            filter_count = filter_count + (remote_addr.to_string().to_lowercase() == r.to_lowercase()) as usize;
        }
    }

    if args.no_system {
        filter_count = filter_count + (socket.pid != 4) as usize;
    }

    filter_count == argc
}

impl Socket {
    pub fn print_socket_table(socket_table: &Vec<Socket>, args: &Args, argc: usize) {
        let mut printable_table = socket_table;
        let filtered_table: Vec<Socket>;
        let mut largest_file_name: usize = 0;
        let mut largest_local_addr: usize = 0;
        let mut largest_remote_addr: usize = 0;
        for socket in socket_table {
            if socket.process_name.len() > largest_file_name{
                largest_file_name = socket.process_name.len();
            }

            if socket.local_addr.len() > largest_local_addr {
                largest_local_addr = socket.local_addr.len();
            }

            if let Some(addr) = &socket.remote_addr {
                if addr.len() > largest_remote_addr {
                    largest_remote_addr = addr.len();
                }
            }
        }
        let pid_w = 10;
        let port_w = 14;
        let process_name_w = std::cmp::max(largest_file_name + 4, 12); // + 4 for some extra padding
        let proto_w = 10;
        let local_addr_w = std::cmp::max(largest_local_addr + 2, 17); // + 2 for some extra padding
        let remote_addr_w = std::cmp::max(largest_remote_addr + 2, 17);
        let state_w  = 15;
        let widths = [pid_w, process_name_w, port_w, proto_w, local_addr_w, remote_addr_w, state_w];
        if argc > 0 {
            filtered_table = printable_table.iter().filter(|socket| filter_socket_table(args, argc, socket)).cloned().collect();
            printable_table = &filtered_table;
        }
        if args.json {
            println!("{}", serde_json::to_string_pretty(&printable_table).unwrap());
        } else {
            print_socket_table_header(&widths);
            for (i, socket) in printable_table.iter().enumerate() {
                print_socket_row(socket, &widths, i);
            }
            print_table_line(&widths);
        }
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let protocol_string = match self.protocol {
            "TCP" => "TCP".bold().blue(),
            "UDP" => "UDP".bold().green(),
            _ => "unknown".bold().red()
        };

        write!(f, "-------------------\nPID: {}\nProcess Name: {}\nPort: {}:{}\nProtocol: {}\nAddress: {}:{}",
            self.pid.to_string().bold(),
            self.process_name.bold().underline(),
            self.port, self.remote_port.unwrap_or(0),
            protocol_string,
            self.local_addr, self.remote_addr.as_deref().unwrap_or("")
        )?;
        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------------------------------

pub fn map_tcp_state(state: u32) -> String {
    match state {
        1 => "CLOSED".to_string(),
        2 => "LISTEN".to_string(),
        3 => "SYN_SENT".to_string(),
        4 => "SYN_RCVD".to_string(),
        5 => "ESTABLISHED".to_string(),
        6 => "FIN_WAIT1".to_string(),
        7 => "FIN_WAIT2".to_string(),
        8 => "CLOSE_WAIT".to_string(),
        9 => "CLOSING".to_string(),
        10 => "LAST_ACK".to_string(),
        11 => "TIME_WAIT".to_string(),
        12 => "DELETE_TCB".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

//--------------------------------------------------------------------------------------------------------------------------