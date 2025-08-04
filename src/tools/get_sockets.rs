use crate::tools::{args::Args, rpderror::RpdError, socket::Socket, tcp_table::*, udp_table::*};

pub fn get_sockets(sockets: &mut Vec<Socket>, args: &Args) {
    let (use_ipv4, use_ipv6) = match args.ip_version {
        Some(4) => (true, false),
        Some(6) => (false, true),
        None => (true, false),
        _ => RpdError::InvalidIpVersionErr().handle()
    };

    let (use_tcp, use_udp) = match args.mode.as_deref().map(|s| s.to_lowercase()) {
        Some(m) if m == "tcp" => (true, false),
        Some(m) if m == "udp" => (false, true),
        None => (true, true),
        Some(m) => RpdError::InvalidProtocolErr(m).handle()

    };

    if use_ipv4 {
        if use_tcp { sockets.extend(get_tcp_sockets()); }
        if use_udp { sockets.extend(get_udp_sockets()); }
    }
    if use_ipv6 {
        if use_tcp { sockets.extend(get_tcp_sockets_ipv6()); }
        if use_udp { sockets.extend(get_udp_sockets_ipv6()); }
    }
}