use std::collections::HashMap;
use colored::{ColoredString, Colorize};
use crate::tools::{print::get_formatted_uptime, socket::Socket};

pub struct Stats {
    connection_count: usize,
    tcp_count: usize,
    udp_count: usize,
    pid_count: usize,
    local_port_count: usize,
    remote_port_count: usize,
    local_address_count: usize,
    remote_address_count: usize,   
    youngest_connection: String,
    oldest_connection: String,
    top_pid: ((u32, String), usize),
    top_remote_address: (String, usize)
}

fn get_socket_stats(socket_table: &Vec<Socket>) -> Stats {
    let connection_count = socket_table.len();
    let mut pid_set: HashMap<(u32, String), usize> = HashMap::new();
    let mut local_addr_set: HashMap<String, usize> = HashMap::new();
    let mut remote_addr_set: HashMap<String, usize> = HashMap::new();
    let mut local_port_set: HashMap<u16, usize> = HashMap::new();
    let mut remote_port_set: HashMap<u16, usize> = HashMap::new();
    
    let udp_count = socket_table.iter().filter(|s| s.protocol == "UDP").count();
    let tcp_count = socket_table.iter().filter(|s| s.protocol == "TCP").count();

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
        if v > top_remote_address.1 && k != "127.0.0.1" && k != "0.0.0.0" {
            top_remote_address = (k.clone(), v);
        }
    });

    Stats {
        connection_count,
        tcp_count,
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

fn visible_length(s: &ColoredString) -> usize {
    strip_ansi_escapes::strip(s.to_string())
        .map(|bytes| String::from_utf8_lossy(&bytes).len())
        .unwrap_or(0)
}

fn print_padded_line(width: usize, left_str: ColoredString, right_str: ColoredString) {
    let left_len = visible_length(&left_str);
    let right_len = visible_length(&right_str);
    let middle_spaces = std::cmp::max(width.saturating_sub(left_len + right_len),1);
    println!("{}{}{}", left_str, " ".repeat(middle_spaces), right_str);
}

pub fn print_socket_stats(socket_table: &Vec<Socket>) {
    let stats = get_socket_stats(socket_table);
    let width: usize = 50;
    print_padded_line(width, String::from("Connections:").bold().underline(), format!("{}", stats.connection_count).bold().blue());
    print_padded_line(width, String::from("  - TCP:").bold(),format!("{}", stats.tcp_count).bold().blue());
    print_padded_line(width, String::from("  - UDP:").bold(),format!("{}", stats.udp_count).bold().blue());
    println!("");
    print_padded_line(width, String::from("Unique PIDs:").bold().underline(), format!("{}", stats.pid_count).bold().blue());
    println!("");
    print_padded_line(width, String::from("Top PID:").bold().underline(), format!("{} ({}) ({})", stats.top_pid.0.0.to_string().bold().cyan(), stats.top_pid.0.1.bold(), stats.top_pid.1.to_string().bold().blue()).normal());
    println!("");
    println!("{}", String::from("Ports:").bold().underline());
    print_padded_line(width, String::from("  - Local ports:").bold(), format!("{}", stats.local_port_count).bold().blue());
    print_padded_line(width, String::from("  - Remote ports:").bold(), format!("{}", stats.remote_port_count).bold().blue());
    println!("");
    println!("{}", String::from("Addresses:").bold().underline());
    print_padded_line(width, String::from("  - Local addreses:").bold(), format!("{}", stats.local_address_count).bold().blue());
    print_padded_line(width, String::from("  - Remote addresses:").bold(), format!("{}", stats.remote_address_count).bold().blue());
    print_padded_line(width, String::from("  - Top remote address:").bold(), format!("{} ({})", stats.top_remote_address.0.bold().cyan(), stats.top_remote_address.1.to_string().bold().blue()).normal(),);
    println!("");
    println!("{}", String::from("Uptimes:").bold().underline());
    print_padded_line(width, String::from("  - Youngest connection:").bold(), format!("{}", stats.youngest_connection).bold().green());
    print_padded_line(width, String::from("  - Local addreses:").bold(), format!("{}", stats.oldest_connection).bold().red());
}