use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::socket::{self, Socket};
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
    match args.ip_version {
        Some(version) => {
            if version == 6 {
                sockets = get_tcp_sockets_ipv6();
                sockets.extend(get_udp_sockets_ipv6());
            } else {
                sockets = get_tcp_sockets();
                sockets.extend(get_udp_sockets());
            }
        }
        None => {
            sockets = get_tcp_sockets();
            sockets.extend(get_udp_sockets());
        }
    }

    if argc > 0 {
        sockets = sockets.iter().filter(|s| socket::filter_socket_table(&args, argc, s)).cloned().collect()
    }

    Socket::sort_socket_table(&mut sockets, &args);
    Socket::print_socket_table(&sockets, &args);
}
