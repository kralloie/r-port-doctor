use clap::Parser;
use r_port_doctor::tools::args::{validate_field_args, Args};
use r_port_doctor::tools::config::{apply_config, get_config, get_config_value, update_config};
use r_port_doctor::tools::dns_lookup::resolve_socket_table_addresses;
use r_port_doctor::tools::get_sockets::get_sockets;
use r_port_doctor::tools::socket::{Socket};
use r_port_doctor::tools::print::print_socket_stats;
fn main() {     
    #[cfg(windows)]
    {
        let _ = enable_ansi_support::enable_ansi_support();
    }

    let mut args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0);
        }
    };
    let config = get_config();


    apply_config(config, &mut args);
    get_config_value(&args.get_config_value);
    update_config(args.set_config_value.clone());

    let argc = args.get_argc();

    let mut sockets: Vec<Socket> = Vec::new();
    get_sockets(&mut sockets, &args);

    validate_field_args(args.fields.as_ref());

    Socket::filter_socket_table(&mut sockets, &args, argc);

    if args.resolve_hostname {
        if !matches!(&args.mode, Some(m) if m.to_lowercase() == "udp") {
            resolve_socket_table_addresses(&mut sockets);
        }
    }

    Socket::sort_socket_table(&mut sockets, &args);
    if args.stats {
        print_socket_stats(&sockets);
    } else {
        Socket::print_socket_table(&sockets, &args);
    }
}
