use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use crate::tools::{args::Args, print::*, range_filter::filter_range};
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
    pub executable_path: Option<String>,
    pub uptime: u64
}

pub const IPV4_ULAF: u32 = AF_INET.0 as u32;
pub const IPV6_ULAF: u32 = AF_INET6.0 as u32;

const TABLE_COLUMNS: usize = 8;
const PID_W: usize = 10;
const PORT_W: usize = 14;
const PROTOCOL_W: usize = 10;
const STATE_W: usize = 15;
const UPTIME_W: usize = 12;
const PROCESS_W: usize = 14;
const LOCAL_ADDR_W: usize = 15;
const REMOTE_ADDR_W: usize = 16;

impl Socket {
    pub fn filter_socket_row (args: &Args, socket: &&Socket) -> bool {
        
        if let Some(range_args) = &args.range {
            if !filter_range(range_args, socket, &args.ip_version) {
                return false
            }
        } 

        if let Some(p) = args.port {
            if socket.port != p {
                return false
            }
        }

        if let Some(p) = args.remote_port {
            if let Some(rp) = socket.remote_port {
                if rp != p {
                    return false
                }
            } else {
                return false
            }       
        }

        if let Some(n) = &args.process_name {
            if !socket.process_name.to_lowercase().contains(n.to_lowercase().as_str()) {
                return false
            }
        }

        if let Some(i) = args.pid {
            if socket.pid != i {
                return false
            }
        }

        if let Some(s) = &args.state {
            if socket.state.to_string().to_lowercase() != s.to_lowercase() {
                return false
            }
        }

        if let Some(l) = &args.local_address {
            if socket.local_addr.to_string().to_lowercase() != l.to_lowercase() {
                return false
            }
        }

        if let Some(r) = &args.remote_address {
            if let Some(remote_addr) = &socket.remote_addr {
                if remote_addr.to_string().to_lowercase() != r.to_lowercase() {
                    return false
                }
            } else {
                return false
            }
        }

        if let Some(s) = args.older_than {
            if socket.uptime < s as u64 {
                return false
            }
        }

        if let Some(s) = args.younger_than {
            if socket.uptime > s as u64 {
                return false
            }
        }

        if args.no_system {
            if socket.pid == 4 {
                return false
            }
        }

        true
    }

    pub fn filter_socket_table(socket_table: &mut Vec<Socket>, args: &Args, argc: usize) {
        if argc > 0 {
            *socket_table = socket_table.iter().filter(|s| Socket::filter_socket_row(&args, s)).cloned().collect();
        }
    }

    pub fn sort_socket_table(socket_table: &mut Vec<Socket>, args: &Args) {
        if let Some(sort_arg) = args.sort_by.clone() {
            let order = sort_arg[0].to_lowercase();
            let field = sort_arg[1].to_lowercase();
            match field.as_str() {
                "pid" => sort_by(order.as_str(), socket_table, |s| s.pid),
                "port" => sort_by(order.as_str(), socket_table, |s| s.port),
                "remote-port" => sort_by(order.as_str(), socket_table, |s| s.remote_port),
                "process-name" => sort_by(order.as_str(), socket_table, |s| s.process_name.clone()),
                "uptime" => sort_by(order.as_str(), socket_table, |s| s.uptime),
                _ => {
                    eprintln!("error: Invalid field argument: '{}'\n\nAvailable arguments:\n\n- pid (Process ID)\n- port (Local Port)\n- remote-port (Remote Port)\n- process-name (Process Name)\n- uptime (Time in seconds since connection started)", field);
                    std::process::exit(0);
                }
            }
        }
    }

    pub fn print_socket_table(socket_table: &Vec<Socket>, args: &Args) {
        let mut largest_file_name: usize = 0;
        let mut largest_local_addr: usize = 0;
        let mut largest_remote_addr: usize = 0;
        let mut largest_uptime: usize = 0;

        socket_table.iter().for_each(|socket| {
            largest_file_name = largest_file_name.max(socket.process_name.len());
            largest_local_addr = largest_local_addr.max(socket.local_addr.len());
            largest_uptime = largest_uptime.max(get_formatted_uptime(&args.uptime_format, socket.uptime).len());
            if let Some(addr) = &socket.remote_addr {
                largest_remote_addr = largest_remote_addr.max(addr.len());
            }
        });
    
        let mut widths: [usize; TABLE_COLUMNS] = [0; TABLE_COLUMNS];
        widths[PID_IDX] = PID_W;
        widths[PORT_IDX] = PORT_W;
        widths[STATE_IDX] = STATE_W;
        widths[PROTOCOL_IDX] = PROTOCOL_W;
        // + 2: Extra padding
        widths[UPTIME_IDX] = std::cmp::max(largest_uptime + 2, UPTIME_W);
        widths[PROCESS_IDX] = std::cmp::max(largest_file_name + 2, PROCESS_W); 
        widths[LOCAL_ADDR_IDX] = std::cmp::max(largest_local_addr + 2, LOCAL_ADDR_W);
        widths[REMOTE_ADDR_IDX] = std::cmp::max(largest_remote_addr + 2, REMOTE_ADDR_W);

        if args.json {
            println!("{}", serde_json::to_string_pretty(&socket_table).unwrap());
        } else {
            print_socket_table_header(&widths, args.compact, &args.fields);
            for socket in socket_table{
                print_socket_row(socket, &widths,args.compact, &args.fields, &args.uptime_format);
            }
            if !args.compact { print_table_line(&widths, &args.fields); }
        }
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

fn sort_by<K: Ord, F: Fn(&Socket) -> K>(order: &str, table: &mut Vec<Socket>, key_field_fn: F) {
    match order {
        "asc" => {
            table.sort_by_key(key_field_fn);
        }
        "desc" => {
            table.sort_by_key(|s| std::cmp::Reverse(key_field_fn(s)));
        }
        _ => {
            eprintln!("error: Invalid order argument: '{}'\n\nAvailable orders:\n  - asc (ascendant)\n  - desc (descendant)", order);
            std::process::exit(0);
        }
    }
}

//--------------------------------------------------------------------------------------------------------------------------