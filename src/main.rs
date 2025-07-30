use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::config::{apply_config, get_config, set_config_value};
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

    let mut args = match Args::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(0);
        }
    };

    let config = get_config();
    if let Some(conf) = config {
        apply_config(conf, &mut args);
    }
    let argc = args.get_argc();

    if let Some(set_conf) = &args.set_config_value {
        let update_config = set_config_value(set_conf[0].as_str(), set_conf.get(1));
        match update_config {
            Ok(()) => println!("Updated configuration: '{}' set to '{}'", set_conf[0], set_conf.get(1).unwrap_or(&String::from("none"))),
            Err(e) => eprintln!("error: {}", e)
        }
        std::process::exit(0)
    }

    let mut sockets: Vec<Socket> = Vec::new();

    get_sockets(&mut sockets, &args);

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
