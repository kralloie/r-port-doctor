use dns_lookup::lookup_addr;
pub fn lookup_address(ip: &str) -> String{
    match ip.trim().parse() {
        Ok(parsed_ip) => {
            match lookup_addr(&parsed_ip) {
                Ok(name) => {
                    name
                }
                _ => {
                    return ip.to_string()
                }
            }
        }
        _ => {
            return ip.to_string()
        }
    }
}