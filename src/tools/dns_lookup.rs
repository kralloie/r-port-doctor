use std::{sync::LazyLock, thread};
use dns_lookup::lookup_addr;
use std::collections::HashSet;
use crate::tools::socket::Socket;
use dashmap::DashMap;

static DNS_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(|| DashMap::new());

pub fn lookup_address(ip: &str) -> String {
    if let Some(cached) = DNS_CACHE.get(ip) {
        return cached.clone()
    }

    let hostname = ip.trim()
        .parse()
        .ok()
        .and_then(|parsed_ip| lookup_addr(&parsed_ip).ok())
        .unwrap_or_else(|| ip.to_string());

    DNS_CACHE.insert(ip.to_string(), hostname.clone());
    hostname
}

pub fn resolve_socket_table_addresses(sockets: &mut Vec<Socket>) {
    let threads = 4;

    let mut addresses_hash_set: HashSet<String> = HashSet::new();

    sockets.iter().for_each(|s| {
        if s.protocol != "UDP" {
            if let Some(addr) = &s.remote_addr {
                addresses_hash_set.insert(addr.clone());
            }   
        }
    });

    if addresses_hash_set.len() == 0 {
        return
    }

    let addresses: Vec<String> = addresses_hash_set.into_iter().collect();
    let len = addresses.len();
    let chunk_size = (len + threads - 1) / threads;

    let mut handles = Vec::new();

    let chunks: Vec<Vec<String>> = addresses
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    for chunk in chunks {
        let handle = thread::spawn(|| {
            for s in chunk {
                lookup_address(s.as_str());
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    sockets.iter_mut().for_each(|s| {
        if s.protocol != "UDP" {
            if let Some(addr) = &s.remote_addr {
                if let Some(resolved_addr) = DNS_CACHE.get(addr) {
                    s.remote_addr = Some(resolved_addr.clone());
                }
            }
        }
    });
}