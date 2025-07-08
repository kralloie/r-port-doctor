use std::path::Path;

use colored::{ColoredString, Colorize};
use crate::tools::socket::Socket;

pub fn print_table_line(widths: &[usize]) {
    let line_string: String = widths
        .iter()
        .map(|w| format!("+{}", "-".repeat(*w)))
        .collect::<String>() + "+";
    println!("{}", line_string);
}

fn ansi_hyperlink(text: &str, url: Option<&str>, width: usize) -> String {
    match url {
        Some(u) => {
            let path = u.replace("\\", "/");
            let path_to_executable_folder = Path::new(&path);
            let padding = " ".repeat(std::cmp::max(width - text.len(), 0));
            if let Some(parent) = path_to_executable_folder.parent() {
                let parent_folder_str = parent.display().to_string();
                format!("{}\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", padding, parent_folder_str, text)
            } else {
                format!("{}\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", padding, u, text)
            }
        }
        None => {
            text.to_string()
        }
    }
}

fn map_state_color(state: &String) -> ColoredString{
    match state.as_str() {
        "CLOSED" => state.red(),
        "LISTEN" => state.cyan(),
        "SYN_SENT" => state.white(),
        "SYN_RCVD" => state.white(),
        "ESTABLISHED"=> state.green(),
        "FIN_WAIT1" => state.white(),
        "FIN_WAIT2" => state.white(),
        "CLOSE_WAIT" => state.yellow(),
        "CLOSING" => state.yellow(),
        "LAST_ACK" => state.white(),
        "TIME_WAIT" => state.white(),
        "DELETE_TCB" => state.white(),
        "UNKNOWN" => state.purple(),
        _ => " ".white()
    }
}

pub fn print_socket_row(socket: &Socket, widths: &[usize], index: usize) {
    let port_str = format!("{}:{}", socket.port, socket.remote_port.unwrap_or(0));
    let remote_addr = socket.remote_addr.as_deref().unwrap_or(" ");
    let protocol_string = match socket.protocol {
        "UDP" => {
            "UDP/IP".bold().blue()
        }
        "TCP" => {
            "TCP/IP".bold().green()
        }
        _ => {
            "unknown".bold().red()
        }
    };
    let process_name = match socket.process_name.as_str() {
        "SYSTEM" => {
            "SYSTEM".bold().cyan()
        }
        "unknown" => {
            "unknown".bold().red()
        }
        _ => {
            ansi_hyperlink(&socket.process_name, socket.executable_path.as_deref(), widths[1]).bold()
        }
    };

    let socket_row_str = format!("{:^pid_w$}|{:>process_name_w$}|{:^port_w$}|{:^proto_w$}|{:>local_addr_w$}|{:>remote_addr_w$}|{:^state_w$}|{:>uptime_w$}",
        socket.pid,
        process_name, 
        port_str, 
        protocol_string,
        socket.local_addr,
        remote_addr,
        map_state_color(&socket.state),
        format!("{}s", socket.uptime),
        pid_w = widths[0],
        process_name_w = widths[1],
        port_w = widths[2],
        proto_w = widths[3],
        local_addr_w = widths[4],
        remote_addr_w = widths[5],
        state_w = widths[6],
        uptime_w = widths[7]
    );
    if index %2==0 {
        println!("|{}|", socket_row_str.on_black());
    } else {
        println!("|{}|", socket_row_str);
    }
}

pub fn print_socket_table_header(widths: &[usize]) {
    print_table_line(widths);
    println!(
        "|{:^pid_w$}|{:^process_name_w$}|{:^port_w$}|{:^proto_w$}|{:^local_addr_w$}|{:^remote_addr_w$}|{:^state_w$}|{:^uptime_w$}|",
        "PID".bold(),
        "Process Name".bold(),
        "Port".bold(),
        "Protocol".bold(),
        "Local Address".bold(),
        "Remote Address".bold(),
        "State".bold(),
        "Uptime".bold(),
        pid_w = widths[0],
        process_name_w = widths[1],
        port_w = widths[2],
        proto_w = widths[3],
        local_addr_w = widths[4],
        remote_addr_w = widths[5],
        state_w = widths[6],
        uptime_w = widths[7]
    );
    print_table_line(widths);
}