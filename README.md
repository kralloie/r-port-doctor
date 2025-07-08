# r-port-doctor

**r-port-doctor** is a high-performance Windows command-line tool written in Rust to inspect open TCP & UDP sockets, along with the processes using them.  
It works like an advanced `netstat` replacement with rich filtering, sorting, and JSON output options.

---

## Features

✅ List all TCP & UDP sockets on IPv4 and IPv6  
✅ Shows local & remote addresses, ports, connection state, owning PID, process name & seconds since the connection started  
✅ Hyperlink on process name to process executable folder (if available)

🔍 Filter by:
- Local/Remote port
- Protocol (TCP / UDP)
- Process name (partial match)
- PID
- Connection state (LISTEN, ESTABLISHED, etc)
- IP version (IPv4/IPv6)
- Local/Remote IP address
  
✅ Sort ascending or descending by PID, process name, port (remote/local) or uptime  
✅ JSON output for easy parsing  
✅ Optionally exclude system processes (PID 4)

---

###  Example output
```
+----------+--------------------------------------+--------------+----------+-----------------+-----------------+---------------+----------+
|   PID    |             Process Name             |     Port     | Protocol |  Local Address  | Remote Address  |     State     |  Uptime  |
+----------+--------------------------------------+--------------+----------+-----------------+-----------------+---------------+----------+
|   1600   |                           svchost.exe|    135:0     |  TCP/IP  |          0.0.0.0|          0.0.0.0|    LISTEN     |    34969s|
|    4     |                                SYSTEM|    139:0     |  TCP/IP  |    11.111.111.11|          0.0.0.0|    LISTEN     |    34969s|
|    4     |                                SYSTEM|    139:0     |  TCP/IP  |     111.111.1.11|          0.0.0.0|    LISTEN     |    34964s|
|   1768   |                    asus_framework.exe|    1042:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |    34515s|
|   1768   |                    asus_framework.exe|  1042:49841  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |    34514s|
|   1768   |                    asus_framework.exe|  1042:49858  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |    34515s|
|   1768   |                    asus_framework.exe|    1043:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |    34515s|
|  24364   |                             Agent.exe|    1120:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |    24303s|
|   3824   |                           svchost.exe|    5040:0    |  TCP/IP  |          0.0.0.0|          0.0.0.0|    LISTEN     |    34838s|
|   9196   |                          postgres.exe|    5432:0    |  TCP/IP  |          0.0.0.0|          0.0.0.0|    LISTEN     |    34958s|
|  17812   |                     SteelSeriesGG.exe|    6327:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |    34513s|
|  17812   |                     SteelSeriesGG.exe|  6327:49868  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |    34513s|
|  17812   |                     SteelSeriesGG.exe|  6327:49869  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |    34513s|
|  17812   |                     SteelSeriesGG.exe|  6327:49900  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |    34513s|
+----------+--------------------------------------+--------------+----------+-----------------+-----------------+---------------+----------+
```
---

## 🔧 Installation

You need Rust installed.  
Clone this repo and build:

```bash
git clone https://github.com/kralloie/r-port-doctor
cd r-port-doctor
cargo build --release
