use std::{collections::HashMap, path::Path, sync::LazyLock};

use colored::{ColoredString, Colorize};
use crate::tools::socket::Socket;

pub const OUTPUT_FIELDS: [&str; 8] = ["pid", "process_name", "port", "protocol", "local_address", "remote_address", "state", "uptime"];
pub const PID_IDX: usize = 0;
pub const PROCESS_IDX: usize = 1;
pub const PORT_IDX: usize = 2;
pub const PROTOCOL_IDX: usize = 3;
pub const LOCAL_ADDR_IDX: usize = 4;
pub const REMOTE_ADDR_IDX: usize = 5;
pub const STATE_IDX: usize = 6;
pub const UPTIME_IDX: usize = 7;

pub static FIELD_WIDTH_MAP: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    let mut map: HashMap<&str, usize> = HashMap::new();
    map.insert("pid", PID_IDX);
    map.insert("process_name", PROCESS_IDX);
    map.insert("port", PORT_IDX);
    map.insert("protocol", PROTOCOL_IDX);
    map.insert("local_address", LOCAL_ADDR_IDX);
    map.insert("remote_address", REMOTE_ADDR_IDX);
    map.insert("state", STATE_IDX);
    map.insert("uptime", UPTIME_IDX);
    map
});

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

pub fn get_formatted_uptime(uptime_arg: &Option<String>, socket_uptime: u64) -> String {
    let hours = socket_uptime / 3600;
    let days = hours / 24;
    let minutes = socket_uptime % 3600 / 60;
    let seconds = socket_uptime % 60;

    let default_uptime_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    if let Some(format) = uptime_arg {
        match format.to_lowercase().as_str() {
            "clock" => {
                default_uptime_str
            }
            "human" => {
                format!("{:2}d {:2}h {:2}m {:2}s", days, hours, minutes, seconds)
            }
            "hours" => {
                format!("{}h", hours)
            }
            "minutes" => {
                format!("{}m", socket_uptime / 60)
            }
            "seconds" => {
                format!("{}s", socket_uptime)
            }
            _ => {
                eprintln!("error: Invalid uptime format");
                std::process::exit(0);
            }
        }
    } else {
        default_uptime_str
    }
}

pub fn print_table_line(widths: &[usize], fields: &Option<Vec<String>>) {
    let mut line_string = String::new();
    if let Some(fields) = fields {
        fields.iter().for_each(|field| {
            if let Some(idx) = FIELD_WIDTH_MAP.get(field.as_str()) {
                if line_string.chars().last() != Some('+') { line_string.push('+'); }
                line_string.push_str(format!("{}+", "-".repeat(widths[*idx])).as_str());
            }
        });
    } else {
        line_string = widths
        .iter()
        .map(|w| format!("+{}", "-".repeat(*w)))
        .collect::<String>() + "+";
    }
    println!("{}", line_string);
}

pub fn print_socket_row(socket: &Socket, widths: &[usize], compact: bool, fields: &Option<Vec<String>>, uptime_arg: &Option<String>) {
    let port_str = format!("{}:{}", socket.port, socket.remote_port.map_or('-'.to_string(), |p| p.to_string()));
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

    let uptime_str = get_formatted_uptime(uptime_arg, socket.uptime);

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
    let mut socket_row_str: String = String::new();

    if let Some(fields) = fields {
        fields.iter()
        .for_each(|field| {
            match field.to_lowercase().as_str() {
                "pid" => {
                    socket_row_str.push_str(format!("{:^pid_w$}|", socket.pid, pid_w = widths[PID_IDX]).as_str());
                }
                "process_name" => {
                    socket_row_str.push_str(format!("{:>process_name_w$}|", process_name, process_name_w = widths[PROCESS_IDX]).as_str());
                }
                "port" => {
                    socket_row_str.push_str(format!("{:^port_w$}|", port_str, port_w = widths[PORT_IDX]).as_str());
                }
                "protocol" => {
                    socket_row_str.push_str(format!("{:^proto_w$}|", protocol_string, proto_w = widths[PROTOCOL_IDX]).as_str());
                }
                "local_address" => {
                    socket_row_str.push_str(format!("{:>local_addr_w$}|", socket.local_addr, local_addr_w = widths[LOCAL_ADDR_IDX]).as_str());
                }
                "remote_address" => {
                    socket_row_str.push_str(format!("{:>remote_addr_w$}|", remote_addr, remote_addr_w = widths[REMOTE_ADDR_IDX]).as_str());
                }
                "state" => {
                    socket_row_str.push_str(format!("{:^state_w$}|", map_state_color(&socket.state), state_w = widths[STATE_IDX]).as_str());
                }
                "uptime" => {
                    socket_row_str.push_str(format!("{:^uptime_w$}|", uptime_str, uptime_w = widths[UPTIME_IDX]).as_str());
                }
                _ => {}
            }
        });
    } else {
        socket_row_str = format!("{:^pid_w$}|{:>process_name_w$}|{:^port_w$}|{:^proto_w$}|{:>local_addr_w$}|{:>remote_addr_w$}|{:^state_w$}|{:^uptime_w$}",
            socket.pid,
            process_name, 
            port_str, 
            protocol_string,
            socket.local_addr,
            remote_addr,
            map_state_color(&socket.state),
            uptime_str,
            pid_w = widths[PID_IDX],
            process_name_w = widths[PROCESS_IDX],
            port_w = widths[PORT_IDX],
            proto_w = widths[PROTOCOL_IDX],
            local_addr_w = widths[LOCAL_ADDR_IDX],
            remote_addr_w = widths[REMOTE_ADDR_IDX],
            state_w = widths[STATE_IDX],
            uptime_w = widths[UPTIME_IDX]
        );
    }

    let row_str = if compact {
        socket_row_str.replace("|", "")
    } else {
        if socket_row_str.chars().last() != Some('|') { socket_row_str.push('|'); }
        format!("|{}", socket_row_str)
    };

    println!("{}", row_str);
    
}

pub fn print_socket_table_header(widths: &[usize], compact: bool, fields: &Option<Vec<String>>) {
    let mut header = String::new();
    if let Some(fields) = fields {
        for field in fields {
            if header.chars().last() != Some('|') { header.push('|') ;}
            let column_header = match field.to_lowercase().as_str() {
                "pid" => {
                    format!("{:^pid_w$}|", "PID".bold(), pid_w = widths[PID_IDX])
                }
                "process_name" => {
                    format!("{:^process_name_w$}|", "Process Name".bold(), process_name_w = widths[PROCESS_IDX])
                }
                "port" => {
                    format!("{:^port_w$}|", "Port".bold(), port_w = widths[PORT_IDX])
                }
                "protocol" => {
                    format!("{:^proto_w$}|", "Protocol".bold(), proto_w = widths[PROTOCOL_IDX])
                }
                "local_address" => {
                    format!("{:^local_addr_w$}|", "Local Address".bold(), local_addr_w = widths[LOCAL_ADDR_IDX])
                }
                "remote_address" => {
                    format!("{:^remote_addr_w$}|", "Remote Address".bold(), remote_addr_w = widths[REMOTE_ADDR_IDX])
                }
                "state" => {
                    format!("{:^state_w$}|", "State".bold(), state_w = widths[STATE_IDX])
                }
                "uptime" => {
                    format!("{:^uptime_w$}|", "Uptime".bold(), uptime_w = widths[UPTIME_IDX])
                }
                _ => continue,
            };
            header.push_str(&column_header);
        };
    } else {
        header = format!("|{:^pid_w$}|{:^process_name_w$}|{:^port_w$}|{:^proto_w$}|{:^local_addr_w$}|{:^remote_addr_w$}|{:^state_w$}|{:^uptime_w$}|",
            "PID".bold(),
            "Process Name".bold(),
            "Port".bold(),
            "Protocol".bold(),
            "Local Address".bold(),
            "Remote Address".bold(),
            "State".bold(),
            "Uptime".bold(),
            pid_w = widths[PID_IDX],
            process_name_w = widths[PROCESS_IDX],
            port_w = widths[PORT_IDX],
            proto_w = widths[PROTOCOL_IDX],
            local_addr_w = widths[LOCAL_ADDR_IDX],
            remote_addr_w = widths[REMOTE_ADDR_IDX],
            state_w = widths[STATE_IDX],
            uptime_w = widths[UPTIME_IDX]
        );
    }

    if compact {
        println!("{}", header.replace("|", ""))
    } else {
        print_table_line(widths, fields);
        println!("{}", header);
        print_table_line(widths, fields);
    }
}