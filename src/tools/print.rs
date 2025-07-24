use colored::{ColoredString, Colorize};
use crate::tools::{print_utils::*, socket::Socket, stats::get_socket_stats};

/////////////////// Socket Table

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
                "process-name" => {
                    socket_row_str.push_str(format!("{:>process_name_w$}|", process_name, process_name_w = widths[PROCESS_IDX]).as_str());
                }
                "port" => {
                    socket_row_str.push_str(format!("{:^port_w$}|", port_str, port_w = widths[PORT_IDX]).as_str());
                }
                "protocol" => {
                    socket_row_str.push_str(format!("{:^proto_w$}|", protocol_string, proto_w = widths[PROTOCOL_IDX]).as_str());
                }
                "local-address" => {
                    socket_row_str.push_str(format!("{:>local_addr_w$}|", socket.local_addr, local_addr_w = widths[LOCAL_ADDR_IDX]).as_str());
                }
                "remote-address" => {
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
                "process-name" => {
                    format!("{:^process_name_w$}|", "Process Name".bold(), process_name_w = widths[PROCESS_IDX])
                }
                "port" => {
                    format!("{:^port_w$}|", "Port".bold(), port_w = widths[PORT_IDX])
                }
                "protocol" => {
                    format!("{:^proto_w$}|", "Protocol".bold(), proto_w = widths[PROTOCOL_IDX])
                }
                "local-address" => {
                    format!("{:^local_addr_w$}|", "Local Address".bold(), local_addr_w = widths[LOCAL_ADDR_IDX])
                }
                "remote-address" => {
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

/////////////////// Stats

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
    println!("");
    print_padded_line(width, String::from("  - TCP:").bold(),format!("{}", stats.tcp_count).bold().blue());
    print_padded_line(width, String::from("  - UDP:").bold(),format!("{}", stats.udp_count).bold().blue());
    print!("\n\n");
    print_padded_line(width, String::from("Unique PIDs:").bold().underline(), format!("{}", stats.pid_count).bold().blue());
    println!("");
    print_padded_line(width, String::from("Top PID:").bold().underline(), format!("{} ({}) ({})", stats.top_pid.0.0.to_string().bold().cyan(), stats.top_pid.0.1.bold(), stats.top_pid.1.to_string().bold().blue()).normal());
    print!("\n\n");
    println!("{}", String::from("Ports:").bold().underline());
    println!("");
    print_padded_line(width, String::from("  - Local ports:").bold(), format!("{}", stats.local_port_count).bold().blue());
    print_padded_line(width, String::from("  - Remote ports:").bold(), format!("{}", stats.remote_port_count).bold().blue());
    print!("\n\n");
    println!("{}", String::from("Addresses:").bold().underline());
    print_padded_line(width, String::from("  - Local addreses:").bold(), format!("{}", stats.local_address_count).bold().blue());
    print_padded_line(width, String::from("  - Remote addresses:").bold(), format!("{}", stats.remote_address_count).bold().blue());
    print_padded_line(width, String::from("  - Top remote address:").bold(), format!("{} ({})", stats.top_remote_address.0.bold().cyan(), stats.top_remote_address.1.to_string().bold().blue()).normal(),);
    print!("\n\n");
    println!("{}", String::from("Uptimes:").bold().underline());
    println!("");
    print_padded_line(width, String::from("  - Youngest connection:").bold(), format!("{}", stats.youngest_connection).bold().green());
    print_padded_line(width, String::from("  - Oldest connection:").bold(), format!("{}", stats.oldest_connection).bold().red());
}