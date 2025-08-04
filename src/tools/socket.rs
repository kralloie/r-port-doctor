use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, str::FromStr};
use regex::Regex;
use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use crate::tools::{args::Args, print::*, print_utils::*, range_filter::{filter_range, validate_range_args, MIN_IPV4, MIN_IPV6}, rpderror::RpdError, validate_address::validate_address};
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
            match Regex::new(&n) {
                Ok(re) => {
                    if !re.is_match(&socket.process_name) {
                        return false
                    }
                }
                Err(_) => {
                    return false
                }
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
            if matches!(&args.local_address, Some(addr) if !validate_address(addr, &args.ip_version)) {
                RpdError::InvalidLocalAddressErr(args.local_address.clone().unwrap()).handle();
            }

            if matches!(&args.remote_address, Some(addr) if !validate_address(addr, &args.ip_version)) {
                RpdError::InvalidRemoteAddressErr(args.remote_address.clone().unwrap()).handle();
            }

            if let Some(range_args) = &args.range {
                validate_range_args(range_args, &args.ip_version);
            }

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
                // Ipv4Addr and Ipv6Addr are mapped into IpAddr so the case returns the same type no matter the IP version
                "local-address" => sort_by(order.as_str(), socket_table, |s| {
                    if matches!(args.ip_version, Some(version) if version == 6) {
                        Ipv6Addr::from_str(&s.local_addr)
                            .map(IpAddr::V6) 
                            .unwrap_or(IpAddr::V6(MIN_IPV6))
                    } else {
                        Ipv4Addr::from_str(&s.local_addr)
                            .map(IpAddr::V4)
                            .unwrap_or(IpAddr::V4(MIN_IPV4))
                    }
                }),
                "remote-address" => sort_by(order.as_str(), socket_table, |s| {
                    if matches!(args.ip_version, Some(version) if version == 6) {
                        s.remote_addr
                            .as_ref()
                            .and_then(|addr| Ipv6Addr::from_str(addr).ok())
                            .map(IpAddr::V6)
                            .unwrap_or(IpAddr::V6(MIN_IPV6))
                    } else {
                        s.remote_addr
                            .as_ref()
                            .and_then(|addr| Ipv4Addr::from_str(addr).ok())
                            .map(IpAddr::V4)
                            .unwrap_or(IpAddr::V4(MIN_IPV4))
                    }
                }),
                _ => RpdError::InvalidSortFieldErr(field).handle()
            }
        }
    }

    pub fn print_socket_table(socket_table: &Vec<Socket>, args: &Args) {
        if args.json {
            println!("{}", serde_json::to_string_pretty(&socket_table).unwrap());
            return
        }

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

        print_socket_table_header(&widths, args.compact, &args.fields);
        for socket in socket_table {
            print_socket_row(socket, &widths,args.compact, &args.fields, &args.uptime_format);
        }
        if !args.compact { print_table_line(&widths, &args.fields); }
    }
}

fn sort_by<K: Ord, F: Fn(&Socket) -> K>(order: &str, table: &mut Vec<Socket>, key_field_fn: F) {
    match order {
        "asc" => table.sort_by_key(key_field_fn),
        "desc" => table.sort_by_key(|s| std::cmp::Reverse(key_field_fn(s))),
        _ => RpdError::InvalidSortOrderErr(order.to_string()).handle()
    }
}