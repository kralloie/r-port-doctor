use std::{sync::LazyLock, thread};
use dns_lookup::lookup_addr;
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
    let len = sockets.len();
    let chunk_size = (len + threads - 1) / threads;

    let mut handles = Vec::new();

    let chunks: Vec<Vec<Socket>> = sockets
        .drain(..)
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    for mut chunk in chunks {
        let handle = thread::spawn(move || {
            for s in &mut chunk {
                if s.protocol == "UDP" {
                    continue;
                }
                if let Some(addr) = &s.remote_addr {
                    s.remote_addr = Some(lookup_address(addr));
                }
            }
            chunk
        });

        handles.push(handle);
    }

    for handle in handles {
        let chunk = handle.join().unwrap();
        sockets.extend(chunk);
    }
}