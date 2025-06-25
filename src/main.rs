use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::socket::{self};
use r_port_doctor::tools::tcp_table::{get_tcp_sockets, get_tcp_sockets_ipv6};
use r_port_doctor::tools::udp_table::{get_udp_sockets, get_udp_sockets_ipv6};
fn main() {     
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Invalid arguments: {}", e);
            std::process::exit(0);
        }
    };
    let argc = args.get_argc();
    let mut sockets: Vec<socket::Socket>;
    match &args.ip_version {
        Some(version) => {
            if *version == 6 {
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

    socket::Socket::print_socket_table(&sockets, &args, argc);
}
