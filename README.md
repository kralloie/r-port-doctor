# r-port-doctor

r-port-doctor is a command-line tool for Windows that provides detailed information about network connections, similar to `netstat`. It allows you to filter, sort, and display information about TCP and UDP sockets, including the process ID (PID), process name, local and remote addresses, and connection state.

## Features ✨

- **Detailed Socket Information:** Get a comprehensive view of TCP and UDP sockets on your system.
- **Filtering:** Filter connections by port, process name, PID, protocol, address and connection state.
- **Sorting:** Sort the output by various fields, including PID, port, process name, address and uptime.
- **JSON Output:** Output the results in JSON format for easy parsing and integration with other tools.
- **Hostname Resolution:** Resolve remote IP addresses to hostnames.
- **Uptime Information:** View the uptime of each connection in different formats.
- **Statistics:** Get a summary of connection statistics, including total connections, TCP/UDP counts, and more.

## Usage 🔧

```
r-port-doctor [OPTIONS]
```

### Options ⚙️

| Long                  | Short | Description                                                                                                 |
| --------------------- | ----- | ----------------------------------------------------------------------------------------------------------- |
| `--port`              | `-l`  | Filter by local port number.                                                                                |
| `--remote-port`       | `-r`  | Filter by remote port number.                                                                               |
| `--mode`              | `-m`  | Filter by protocol (TCP or UDP).                                                                            |
| `--process-name`      | `-n`  | Filter by process name (regular expression).                                                                |
| `--pid`               | `-i`  | Filter by process ID (PID).                                                                                 |
| `--state`             | `-s`  | Filter by connection state (e.g., LISTEN, ESTABLISHED).                                                     |
| `--ip-version`        | `-v`  | Specify IP version 4 (IPv4) or 6 (IPv6). Defaults to IPv4.                                                  |
| `--local-address`     |       | Filter by local IP address.                                                                                 |
| `--remote-address`    |       | Filter by remote IP address.                                                                                |
| `--json`              |       | Output results in JSON format.                                                                              |
| `--no-system`         |       | Exclude system processes (e.g., PID 4) from the output.                                                     |
| `--sort`              |       | Sort output in the specified order by the specified field.                                                  |
| `--resolve-hostname`  |       | Resolve remote IP addresses to hostnames using DNS.                                                         |
| `--compact`           |       | Removes table borders from output.                                                                          |
| `--older-than`        |       | Filter connections by uptime being older than provided seconds.                                             |
| `--younger-than`      |       | Filter connections by uptime being younger than provided seconds.                                           |
| `--fields`            |       | Show only the specified fields in the table.                                                                |
| `--uptime-format`     |       | Specify uptime format (clock, human, hours, minutes, seconds).                                              |
| `--range`             |       | Filter rows by value ranges of the specified field.                                                         |
| `--stats`             |       | Outputs various statistics related to the socket table instead of the table itself.                         |

### Available Fields for Arguments 📋

- **`--sort`**:
  - `pid`
  - `port`
  - `remote-port`
  - `process-name`
  - `uptime`
  - `local-address`
  - `remote-address`

- **`--fields`**:
  - `pid`
  - `process-name`
  - `port`
  - `protocol`
  - `local-address`
  - `remote-address`
  - `state`
  - `uptime`

- **`--range`**:
  - `pid`
  - `port`
  - `remote-port`
  - `uptime` (in seconds)
  - `local-address`
  - `remote-address`

## Examples 💡

- **List all TCP connections:**
  ```bash
  r-port-doctor -m tcp
  ```

- **Find the process using port 443:**
  ```bash
  r-port-doctor -l 443
  ```

- **List all connections for a specific process:**
  ```bash
  r-port-doctor -n "chrome.exe"
  ```

- **Sort connections by uptime in descending order:**
  ```bash
  r-port-doctor --sort desc uptime
  ```

- **Output connections in JSON format:**
  ```bash
  r-port-doctor --json
  ```

- **Filter connections with a local port range between 8000 and 9000:**
  ```bash
  r-port-doctor --range port 8000 9000
  ```

- **Filter connections with a remote IPv4 address range:**
  ```bash
  r-port-doctor --range remote-address 192.168.1.1 192.168.1.255
  ```
## Installation 📦

1. Clone the repository:
   ```bash
   git clone https://github.com/kralloie/r-port-doctor.git
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. The executable will be located in the `target/release` directory.

## Contributing 🤝

Contributions are welcome! Please feel free to open an issue.
