use clap::Parser;
use r_port_doctor::tools::args::Args;
use r_port_doctor::tools::socket;
fn main() {     
    let args = Args::parse();
    let argc = args.get_argc();
    let mut sockets = socket::get_tcp_sockets(socket::IPV4_ULAF);
    sockets.extend(socket::get_udp_sockets(socket::IPV4_ULAF));

    socket::Socket::print_socket_table(&sockets, &args, argc);
}
