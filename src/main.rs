use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::dns_lookup::resolve_socket_table_addresses;
use r_port_doctor::tools::get_sockets::get_sockets;
use r_port_doctor::tools::print_utils::OUTPUT_FIELDS;
use r_port_doctor::tools::socket::{Socket};
use r_port_doctor::tools::print::print_socket_stats;
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

    if let Some(fields) = &args.fields {
        let mut seen_fields = std::collections::HashSet::new();
        for field in fields {
            let lower_field = field.to_lowercase();
            if !seen_fields.insert(lower_field.clone()) {
                eprintln!("error: Repeated field '{}'", field);
                std::process::exit(0);
            }
            if !OUTPUT_FIELDS.contains(&lower_field.as_str()) {
                eprintln!("error: Invalid field: '{}'\n\nAvailable fields:\n\n- pid\n- process_name\n- port\n- protocol\n- local_address\n- remote_address\n- state\n- uptime", field);
                std::process::exit(0);
            }
        }
    }
    
    Socket::filter_socket_table(&mut sockets, &args, argc);
    Socket::sort_socket_table(&mut sockets, &args);
    if args.stats {
        print_socket_stats(&sockets);
    } else {
        Socket::print_socket_table(&sockets, &args);
    }
}
