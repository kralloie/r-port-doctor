use std::{collections::HashMap, sync::{LazyLock, Mutex}};

use dns_lookup::lookup_addr;

use crate::tools::socket::Socket;

static DNS_CACHE: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn lookup_address(ip: &str) -> String {
    let mut cache = DNS_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(ip) {
        return cached.clone()
    }

    let hostname = ip.trim()
        .parse()
        .ok()
        .and_then(|parsed_ip| lookup_addr(&parsed_ip).ok())
        .unwrap_or_else(|| ip.to_string());

    cache.insert(ip.to_string(), hostname.clone());
    hostname
}

pub fn resolve_socket_table_addresses(sockets: &mut Vec<Socket>) {
    sockets.iter_mut().for_each(|s|  {
        if s.protocol == "UDP" {
            return
        }
        if let Some(addr) = &s.remote_addr {
            s.remote_addr = Some(lookup_address(&addr));
        }
    });
}