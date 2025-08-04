use colored::Colorize;

pub enum RpdError {
    ParseArgsErr(String),

    // '--fields' arg errors
    RepeatedFieldArgErr(String),
    InvalidFieldArgErr(String),

    // Config file errors
    ConfigDirNotFoundErr(),
    InvalidConfigKeyErr(String),
    UpdateConfigErr(String),

    InvalidIpVersionErr(),

    InvalidProtocolErr(String),

    InvalidUptimeFormatErr(String),

    // Range filtering errors
    InvalidRangeFieldErr(String),
    InvalidRangeMinErr(String, String),
    InvalidRangeMaxErr(String, String),

    // IP Address errors
    InvalidLocalAddressErr(String),
    InvalidRemoteAddressErr(String),

    // Sort asc/desc errors
    InvalidSortFieldErr(String),
    InvalidSortOrderErr(String)
}

impl RpdError {
    pub fn handle(&self) -> ! {
        match self {
            RpdError::ParseArgsErr(err) => eprintln!("{}", err),
            RpdError::RepeatedFieldArgErr(repeated_field) => eprintln!("error: Repeated field '{}'", repeated_field.bold().underline()),
            RpdError::InvalidFieldArgErr(invalid_field) => eprintln!("error: Invalid field: '{}'\n\nAvailable fields:\n\n  - pid\n  - process_name\n  - port\n  - protocol\n  - local_address\n  - remote_address\n  - state\n  - uptime", invalid_field.bold().underline()),
            RpdError::ConfigDirNotFoundErr() => eprintln!("error: Config directory not found"),
            RpdError::InvalidConfigKeyErr(invalid_key) => eprintln!("error: Invalid configuration key: '{}'\n\nUse '--help' to see available configurations or read the configuration file on 'AppData\\Roaming\\r-port-doctor\\config.toml'", invalid_key.bold().underline()),
            RpdError::UpdateConfigErr(err) => eprintln!("error: {}", err),
            RpdError::InvalidIpVersionErr() => eprintln!("error: Invalid IP version\n\nValid versions:\n\n  - 4 (IPv4)\n  - 6 (IPv6)"),
            RpdError::InvalidProtocolErr(invalid_protocol) => eprintln!("error: Invalid protocol: '{}'\n\nAvailable protocols:\n\n  - TCP\n  - UDP", invalid_protocol.bold().underline()),
            RpdError::InvalidUptimeFormatErr(invalid_uptime_format) => eprintln!("error: Invalid uptime format: '{}'\n\nAvailable formats:\n\n  - clock\n  - human\n  - hours\n  - minutes\n  - seconds", invalid_uptime_format.bold().underline()),
            RpdError::InvalidRangeFieldErr(invalid_range_field) => eprintln!("error: Invalid <FIELD> value '{}' provided for range filtering", invalid_range_field.bold().underline()),
            RpdError::InvalidRangeMinErr(invalid_range_min, range_field) => eprintln!("error: Invalid <MIN> value '{}' provided for {} range filtering", invalid_range_min.bold().underline(), range_field.bold().underline()),
            RpdError::InvalidRangeMaxErr(invalid_range_max, range_field ) => eprintln!("error: Invalid <MAX> value '{}' provided for {} range filtering", invalid_range_max.bold().underline(), range_field.bold().underline()),
            RpdError::InvalidLocalAddressErr(invalid_address) => eprintln!("error: Invalid local address provided: '{}'", invalid_address.bold().underline()),
            RpdError::InvalidRemoteAddressErr(invalid_address) => eprintln!("error: Invalid remote address provided: '{}'", invalid_address.bold().underline()),
            RpdError::InvalidSortFieldErr(invalid_order_field) => eprintln!("error: Invalid sort field argument: '{}'\n\nAvailable arguments:\n\n  - pid (Process ID)\n  - port (Local Port)\n  - remote-port (Remote Port)\n  - process-name (Process Name)\n  - uptime (Time in seconds since connection started)", invalid_order_field.bold().underline()),
            RpdError::InvalidSortOrderErr(invalid_order_arg) => eprintln!("error: Invalid sort order argument: '{}'\n\nAvailable orders:\n  - asc (ascendant)\n  - desc (descendant)", invalid_order_arg.bold().underline()),
        }
        std::process::exit(0);
    }
}