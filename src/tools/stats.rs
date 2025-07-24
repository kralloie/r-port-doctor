use std::collections::HashMap;
use crate::tools::{print_utils::get_formatted_uptime, socket::Socket};

pub struct Stats {
    pub connection_count: usize,
    pub tcp_count: usize,
    pub established_count: usize,
    pub listen_count: usize,
    pub udp_count: usize,
    pub pid_count: usize,
    pub local_port_count: usize,
    pub remote_port_count: usize,
    pub local_address_count: usize,
    pub remote_address_count: usize,   
    pub youngest_connection: String,
    pub oldest_connection: String,
    pub top_pid: ((u32, String), usize),
    pub top_remote_address: (String, usize)
}

pub fn get_socket_stats(socket_table: &Vec<Socket>) -> Stats {
    let connection_count = socket_table.len();
    let mut pid_set: HashMap<(u32, String), usize> = HashMap::new();
    let mut local_addr_set: HashMap<String, usize> = HashMap::new();
    let mut remote_addr_set: HashMap<String, usize> = HashMap::new();
    let mut local_port_set: HashMap<u16, usize> = HashMap::new();
    let mut remote_port_set: HashMap<u16, usize> = HashMap::new();
    
    let udp_count = socket_table.iter().filter(|s| s.protocol == "UDP").count();
    let tcp_count = socket_table.iter().filter(|s| s.protocol == "TCP").count();
    let established_count = socket_table.iter().filter(|s| s.state == "ESTABLISHED").count();
    let listen_count = socket_table.iter().filter(|s| s.state == "LISTEN").count();

    let mut youngest_connection: u64 = u64::MAX;
    let mut oldest_connection: u64 = u64::MIN;

    let mut top_pid: ((u32, String), usize) = ((0, String::from("")), 0);
    let mut top_remote_address: (String, usize) = (String::from(""), 0);

    socket_table.iter().for_each(|s| {
        *pid_set.entry((s.pid, s.process_name.clone())).or_insert(0) += 1;

        *local_port_set.entry(s.port).or_insert(0) += 1;
        if let Some(port) = s.remote_port {
            *remote_port_set.entry(port).or_insert(0) += 1;
        }
        
        *local_addr_set.entry(s.local_addr.clone()).or_insert(0) += 1;
        if let Some(addr) = &s.remote_addr {
            *remote_addr_set.entry(addr.clone()).or_insert(0) += 1;
        }

        youngest_connection = std::cmp::min(youngest_connection, s.uptime);
        oldest_connection = std::cmp::max(oldest_connection, s.uptime);
    });

    pid_set.iter().for_each(|(k, &v)| {
        if v > top_pid.1 {
            top_pid = (k.clone(), v);
        }
    });

    remote_addr_set.iter().for_each(|(k, &v)| {
        if v > top_remote_address.1 && k != "127.0.0.1" && k != "0.0.0.0" && k != "::" {
            top_remote_address = (k.clone(), v);
        }
    });

    Stats {
        connection_count,
        tcp_count,
        established_count,
        listen_count,
        udp_count,
        pid_count: pid_set.len(),
        local_port_count: local_port_set.len(),
        remote_port_count: remote_port_set.len(),
        local_address_count: local_addr_set.len(),
        remote_address_count: remote_addr_set.len(),
        youngest_connection: get_formatted_uptime(&Some("human".to_string()), youngest_connection),
        oldest_connection: get_formatted_uptime(&Some("human".to_string()), oldest_connection),
        top_pid,
        top_remote_address
    }
}