use dns_lookup::lookup_addr;

use crate::tools::socket::Socket;
pub fn lookup_address(ip: &str) -> String{
    match ip.trim().parse() {
        Ok(parsed_ip) => {
            match lookup_addr(&parsed_ip) {
                Ok(name) => {
                    name
                }
                _ => {
                    ip.to_string()
                }
            }
        }
        _ => {
            ip.to_string()
        }
    }
}

pub fn resolve_socket_table_addresses(sockets: &mut Vec<Socket>) {
    sockets.iter_mut().for_each(|s|  {
        if let Some(addr) = &s.remote_addr {
            s.remote_addr = Some(lookup_address(&addr));
        }
    });
}