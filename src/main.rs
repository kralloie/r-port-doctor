use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::dns_lookup::resolve_socket_table_addresses;
use r_port_doctor::tools::get_sockets::get_sockets;
use r_port_doctor::tools::socket::{Socket};
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

    let mut sockets: Vec<Socket> = Vec::new();

    get_sockets(&mut sockets, &args);

    if args.resolve_hostname {
        if !matches!(&args.mode, Some(m) if m.to_lowercase() == "udp") {
            resolve_socket_table_addresses(&mut sockets);
        }
    }
    
    Socket::filter_socket_table(&mut sockets, &args, argc);
    Socket::sort_socket_table(&mut sockets, &args);
    Socket::print_socket_table(&sockets, &args);
}
