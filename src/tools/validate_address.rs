use std::{net::{Ipv4Addr, Ipv6Addr}, str::FromStr};

pub fn validate_address(address: &String, version: &Option<u8>) -> bool {
    if let Some(v) = version {
        match v {
            4 => Ipv4Addr::from_str(&address).is_ok(),
            6 => Ipv6Addr::from_str(&address).is_ok(),
            _ => false
        }
    } else {
        Ipv4Addr::from_str(&address).is_ok()
    }
}