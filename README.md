# r-port-doctor

**r-port-doctor** is a high-performance Windows command-line tool written in Rust to inspect open TCP & UDP sockets, along with the processes using them.  
It works like an advanced `netstat` replacement with rich filtering, sorting, and JSON output options.

---

## Features

✅ List all TCP & UDP sockets on IPv4 and IPv6  
✅ Shows local & remote addresses, ports, connection state, owning PID & process name  
✅ Hyperlink on process name to process executable folder  

🔍 Filter by:
- Local/Remote port
- Protocol (TCP / UDP)
- Process name (partial match)
- PID
- Connection state (LISTEN, ESTABLISHED, etc)
- IP version (IPv4/IPv6)
- Local/Remote IP address
  
✅ Sort ascending or descending by PID, process name, or port  
✅ JSON output for easy parsing  
✅ Optionally exclude system processes (PID 4)

---

###  Example output
```
+----------+-------------------------+--------------+----------+-----------------+-----------------+---------------+
|   PID    |      Process Name       |     Port     | Protocol |  Local Address  | Remote Address  |     State     |
+----------+-------------------------+--------------+----------+-----------------+-----------------+---------------+
|    4     |                   SYSTEM|    139:0     |  TCP/IP  |    11.111.111.11|          0.0.0.0|    LISTEN     |
|    4     |                   SYSTEM|    139:0     |  TCP/IP  |     111.111.1.11|          0.0.0.0|    LISTEN     |
|    4     |                   SYSTEM|   27339:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |
|    4     |                   SYSTEM|    445:0     |  TCP/IP  |          0.0.0.0|          0.0.0.0|    LISTEN     |
|   8832   |     SteelSeriesPrism.exe|   49900:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |
|   8832   |     SteelSeriesPrism.exe| 49900:49908  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|   8832   |     SteelSeriesPrism.exe| 49901:49735  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|   8832   |     SteelSeriesPrism.exe|  49902:6327  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|   8832   |     SteelSeriesPrism.exe| 49903:49735  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|   8832   |     SteelSeriesPrism.exe|  49905:6327  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|  17948   |    SteelSeriesEngine.exe|   49735:0    |  TCP/IP  |        127.0.0.1|          0.0.0.0|    LISTEN     |
|  17948   |    SteelSeriesEngine.exe| 49735:49903  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
|  17948   |    SteelSeriesEngine.exe| 49735:49907  |  TCP/IP  |        127.0.0.1|        127.0.0.1|  ESTABLISHED  |
+----------+-------------------------+--------------+----------+-----------------+-----------------+---------------+
```
---

## 🔧 Installation

You need Rust installed.  
Clone this repo and build:

```bash
git clone https://github.com/kralloie/r-port-doctor
cd r-port-doctor
cargo build --release
