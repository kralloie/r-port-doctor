use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::dns_lookup::resolve_socket_table_addresses;
use r_port_doctor::tools::socket::{Socket};
use r_port_doctor::tools::tcp_table::{get_tcp_sockets, get_tcp_sockets_ipv6};
use r_port_doctor::tools::udp_table::{get_udp_sockets, get_udp_sockets_ipv6};
fn main() {     
    #[cfg(windows)]
    {
        let _ = enable_ansi_support::enable_ansi_support();
    }

    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0);
        }
    };
    let argc = args.get_argc();

    let mut sockets: Vec<Socket>;

    match (args.ip_version, args.mode.as_deref().map(|s| s.to_lowercase())) {
        (Some(6), Some(m)) => match m.as_str() {
            "udp" => sockets = get_udp_sockets_ipv6(),
            "tcp" => sockets = get_tcp_sockets_ipv6(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (Some(6), None) => {
            sockets = get_tcp_sockets_ipv6();
            sockets.extend(get_udp_sockets_ipv6());
        }
        (Some(4), Some(m)) => match m.as_str() {
            "udp" => sockets = get_udp_sockets(),
            "tcp" => sockets = get_tcp_sockets(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (Some(4), None) => {
            sockets = get_tcp_sockets();
            sockets.extend(get_udp_sockets());
        }
        (Some(_), _) => {
            eprintln!("error: Invalid IP version\n\nValid versions:\n\n- 4 (IPv4)\n- 6 (IPv6)");
            std::process::exit(0);
        }
        (None, Some(m)) => match m.as_str() {
            "udp" => sockets = get_udp_sockets(),
            "tcp" => sockets = get_tcp_sockets(),
            _ => {
                eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n- TCP\n- UDP", m);
                std::process::exit(0);
            }
        },
        (None, None) => {
            sockets = get_tcp_sockets();
            sockets.extend(get_udp_sockets());
        }
    }

    if args.resolve_hostname {
        if !matches!(&args.mode, Some(m) if m.to_lowercase() == "udp") {
            resolve_socket_table_addresses(&mut sockets);
        }
    }
    
    Socket::filter_socket_table(&mut sockets, &args, argc);
    Socket::sort_socket_table(&mut sockets, &args);
    Socket::print_socket_table(&sockets, &args);
}
