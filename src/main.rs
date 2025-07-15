use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::dns_lookup::resolve_socket_table_addresses;
use r_port_doctor::tools::get_sockets::get_sockets;
use r_port_doctor::tools::print::OUTPUT_FIELDS;
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

    if let Some(fields) = &args.fields {
        let mut selected_fields: Vec<String> = Vec::new();
        fields.iter().for_each(|field| {
            let field = field.to_lowercase();
            if selected_fields.contains(&field) {
                eprintln!("error: Repeated field '{}'", field);
                std::process::exit(0);
            }
            if !OUTPUT_FIELDS.contains(&field.as_str()) {
                eprintln!("error: Invalid field: '{}'\n\nAvailable fields:\n\n- pid\n- process_name\n- port\n- protocol\n- local_address\n- remote_address\n- state\n- uptime", field);
                std::process::exit(0);
            } else {
                selected_fields.push(field.clone());
            }
        });
    }
    
    Socket::filter_socket_table(&mut sockets, &args, argc);
    Socket::sort_socket_table(&mut sockets, &args);
    Socket::print_socket_table(&sockets, &args);
}
