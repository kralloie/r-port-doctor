use std::{net::{Ipv4Addr, Ipv6Addr}, str::FromStr};
use crate::tools::socket::Socket;

pub const MIN_IPV4: Ipv4Addr = Ipv4Addr::new(0,0,0,0);
pub const MAX_IPV4: Ipv4Addr = Ipv4Addr::new(255,255,255,255);
pub const MIN_IPV6: Ipv6Addr = Ipv6Addr::UNSPECIFIED;
pub const MAX_IPV6: Ipv6Addr = Ipv6Addr::new(0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF);

pub fn check_field_range<T: FromStr + Copy + PartialOrd>(default_min: T, default_max: T, range_args: &Vec<String>, socket_field: T) -> bool {
    let min = range_args.get(1)
        .map(|s| s.parse::<T>().unwrap_or(default_min))
        .unwrap_or(default_min);
    let max = range_args.get(2)
        .map(|s| s.parse::<T>().unwrap_or(default_max))
        .unwrap_or(default_max);
    socket_field >= min && socket_field <= max
}

pub fn check_address_range<T: FromStr + Copy + PartialOrd>(default_min: T, default_max: T, range_args: &Vec<String>, socket_field: &String) -> bool {
    let get_addr = T::from_str(socket_field.as_str());
    match get_addr {
        Ok(addr) => check_field_range(default_min, default_max, range_args, addr),
        Err(_) => false
    }
}

pub fn filter_range(range_args: &Vec<String>, socket: &&Socket, ip_version: &Option<u8>) -> bool {
    match range_args[0].to_lowercase().as_str() {
        "pid" => check_field_range(u32::MIN, u32::MAX, range_args, socket.pid),
        "port" => check_field_range(u16::MIN, u16::MAX, range_args, socket.port), 
        "remote-port" => {
            if let Some(remote_port) = socket.remote_port {
                check_field_range(u16::MIN, u16::MAX, range_args, remote_port)
            } else {
                false
            }
        }
        "uptime" => check_field_range(u64::MIN, u64::MAX, range_args, socket.uptime),
        "local-address" => {
            if let Some(version) = ip_version {
                match version {
                    4 => check_address_range(MIN_IPV4, MAX_IPV4, range_args, &socket.local_addr),
                    6 => check_address_range(MIN_IPV6, MAX_IPV6, range_args, &socket.local_addr),
                    _ => false
                }
            } else {
                check_address_range(MIN_IPV4, MAX_IPV4, range_args, &socket.local_addr)
            }
        }
        "remote-address" => {
            if let Some(addr) = &socket.remote_addr {
                if let Some(version) = ip_version {
                    match version {
                        4 => check_address_range(MIN_IPV4, MAX_IPV4, range_args, addr),
                        6 => check_address_range(MIN_IPV6, MAX_IPV6, range_args, addr),
                        _ => false
                    }
                } else {
                    check_address_range(MIN_IPV4, MAX_IPV4, range_args, addr)
                }
            } else {
                false
            }
        }
        _ => {
            false
        }
    }
}