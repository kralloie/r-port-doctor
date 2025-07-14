use crate::tools::{args::Args, socket::Socket, udp_table::*, tcp_table::*};

pub fn get_sockets(sockets: &mut Vec<Socket>, args: &Args) {
    match (args.ip_version, args.mode.as_deref().map(|s| s.to_lowercase())) {
        (Some(6), Some(m)) => match m.as_str() {
            "udp" => *sockets = get_udp_sockets_ipv6(),
            "tcp" => *sockets = get_tcp_sockets_ipv6(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (Some(6), None) => {
            *sockets = get_tcp_sockets_ipv6();
            sockets.extend(get_udp_sockets_ipv6());
        }
        (Some(4), Some(m)) => match m.as_str() {
            "udp" => *sockets = get_udp_sockets(),
            "tcp" => *sockets = get_tcp_sockets(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (Some(4), None) => {
            *sockets = get_tcp_sockets();
            sockets.extend(get_udp_sockets());
        }
        (Some(_), _) => {
            eprintln!("error: Invalid IP version\n\nValid versions:\n\n- 4 (IPv4)\n- 6 (IPv6)");
            std::process::exit(0);
        }
        (None, Some(m)) => match m.as_str() {
            "udp" => *sockets = get_udp_sockets(),
            "tcp" => *sockets = get_tcp_sockets(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (None, None) => {
            *sockets = get_tcp_sockets();
            sockets.extend(get_udp_sockets());
        }
    }
}