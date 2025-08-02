use std::{collections::HashMap, path::Path, sync::LazyLock};
use colored::{ColoredString, Colorize};

pub const OUTPUT_FIELDS: [&str; 8] = ["pid", "process-name", "port", "protocol", "local-address", "remote-address", "state", "uptime"];
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
    map.insert("process-name", PROCESS_IDX);
    map.insert("port", PORT_IDX);
    map.insert("protocol", PROTOCOL_IDX);
    map.insert("local-address", LOCAL_ADDR_IDX);
    map.insert("remote-address", REMOTE_ADDR_IDX);
    map.insert("state", STATE_IDX);
    map.insert("uptime", UPTIME_IDX);
    map
});

pub fn ansi_hyperlink(text: &str, url: Option<&str>, width: usize) -> String {
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

pub fn get_formatted_uptime(uptime_arg: &Option<String>, socket_uptime: u64) -> String {
    let hours = socket_uptime / 3600;
    let days = hours / 24;
    let minutes = socket_uptime % 3600 / 60;
    let seconds = socket_uptime % 60;

    let default_uptime_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    if let Some(format) = uptime_arg {
        match format.to_lowercase().as_str() {
            "clock" => default_uptime_str,
            "human" => format!("{:2}d {:2}h {:2}m {:2}s", days, hours, minutes, seconds),
            "hours" => format!("{}h", hours),
            "minutes" => format!("{}m", socket_uptime / 60),
            "seconds" => format!("{}s", socket_uptime),
            _ => {
                eprintln!("error: Invalid uptime format: '{}'\n\nAvailable formats:\n\n  - clock\n  - human\n  - hours\n  - minutes\n  - seconds", format.bold().underline());
                std::process::exit(0);
            }
        }
    } else {
        default_uptime_str
    }
}

pub fn map_state_color(state: &String) -> ColoredString{
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

pub fn visible_length(s: &ColoredString) -> usize {
    strip_ansi_escapes::strip(s.to_string())
        .map(|bytes| {
            String::from_utf8_lossy(&bytes)
                .chars()
                .filter(|c| *c != '\n' && *c != '\r')
                .count()
        })
        .unwrap_or(0)
}