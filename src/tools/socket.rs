use std::fmt::{Display, Formatter};
use windows::Win32::{
        Foundation::NO_ERROR, NetworkManagement::IpHelper::{GetExtendedTcpTable, GetExtendedUdpTable, MIB_TCPROW_OWNER_MODULE, MIB_TCPTABLE_OWNER_MODULE, MIB_UDPROW_OWNER_MODULE, MIB_UDPTABLE_OWNER_MODULE, TCP_TABLE_OWNER_MODULE_ALL, UDP_TABLE_OWNER_MODULE}, Networking::WinSock::{AF_INET, AF_INET6}, System::{
            ProcessStatus::GetProcessImageFileNameW,
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        }
    };
use std::{net::Ipv4Addr};
use std::{ffi::c_void, ptr};
use colored::*;
use crate::tools::{args::Args, nt_to_dos::to_dos_path, print::*};

//--------------------------------------------------------------------------------------------------------------------------
#[derive(Clone)]
pub struct Socket {
    pub process_name: String,
    pub pid: u32,
    pub port: u16,
    pub protocol: &'static str,
    pub local_addr: String,
    pub remote_addr: Option<String>,
    pub remote_port: Option<u16>,
    pub state: String,
    pub executable_path: Option<String>
}

pub const IPV4_ULAF: u32 = AF_INET.0 as u32;
pub const IPV6_ULAF: u32 = AF_INET6.0 as u32;

fn filter_socket_table (args: &Args, argc: usize, socket: &&Socket) -> bool {
    let mut filter_count = 0;
    if let Some(p) = args.port {
        filter_count = filter_count + (socket.port == p) as usize;
    }

    if let Some(m) = args.mode.clone() {
        filter_count = filter_count + (socket.protocol.to_lowercase() == m.to_lowercase()) as usize;
    }

    if let Some(n) = args.processname.clone() {
        filter_count = filter_count + (socket.process_name.to_lowercase().contains(n.to_lowercase().as_str())) as usize;
    }

    if let Some(i) = args.pid {
        filter_count = filter_count + (socket.pid == i) as usize;
    }

    if let Some(s) = args.state.clone() {
        filter_count = filter_count + (socket.state.to_string().to_lowercase() == s.to_lowercase()) as usize;
    }

    filter_count == argc
}

impl Socket {
    pub fn print_socket_table(socket_table: &Vec<Socket>, args: &Args, argc: usize) {
        let mut printable_table = socket_table;
        let filtered_table: Vec<Socket>;
        let mut largest_file_name: usize = 0;
        for socket in socket_table {
            if socket.process_name.len() > largest_file_name{
                largest_file_name = socket.process_name.len();
            }
        }
        let pid_w = 10;
        let port_w = 14;
        let process_name_w = std::cmp::max(largest_file_name + 4, 12);
        let proto_w = 10;
        let local_addr_w = 17;
        let remote_addr_w = 17;
        let state_w  = 15;
        let widths = [pid_w, process_name_w, port_w, proto_w, local_addr_w, remote_addr_w, state_w];
        print_socket_table_header(&widths);
        if argc > 0 {
            filtered_table = printable_table.iter().filter(|socket| filter_socket_table(args, argc, socket)).cloned().collect();
            printable_table = &filtered_table;
        }
        for (i, socket) in printable_table.iter().enumerate() {
            print_socket_row(socket, &widths, i);
        }
        print_table_line(&widths);
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let protocol_string = match self.protocol {
            "TCP" => "TCP".bold().blue(),
            "UDP" => "UDP".bold().green(),
            _ => "unknown".bold().red()
        };

        write!(f, "-------------------\nPID: {}\nProcess Name: {}\nPort: {}:{}\nProtocol: {}\nAddress: {}:{}",
            self.pid.to_string().bold(),
            self.process_name.bold().underline(),
            self.port, self.remote_port.unwrap_or(0),
            protocol_string,
            self.local_addr, self.remote_addr.as_deref().unwrap_or("")
        )?;
        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------------------------------

fn map_tcp_state(state: u32) -> String {
    match state {
        1 => "CLOSED".to_string(),
        2 => "LISTEN".to_string(),
        3 => "SYN_SENT".to_string(),
        4 => "SYN_RCVD".to_string(),
        5 => "ESTABLISHED".to_string(),
        6 => "FIN_WAIT1".to_string(),
        7 => "FIN_WAIT2".to_string(),
        8 => "CLOSE_WAIT".to_string(),
        9 => "CLOSING".to_string(),
        10 => "LAST_ACK".to_string(),
        11 => "TIME_WAIT".to_string(),
        12 => "DELETE_TCB".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}

//--------------------------------------------------------------------------------------------------------------------------

pub fn get_udp_sockets(ulaf: u32) -> Vec<Socket> {
    let mut udp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedUdpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            ulaf,
            UDP_TABLE_OWNER_MODULE,
            0,
        )
    };

    if result != NO_ERROR.0 {
        if result == windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.0 {
            let mut buffer = vec![0u8; table_size as usize];
            let final_result = unsafe {
                GetExtendedUdpTable(
                    Some(buffer.as_mut_ptr() as *mut c_void),
                    &mut table_size,
                    false,
                    ulaf,
                    UDP_TABLE_OWNER_MODULE,
                    0,
                )
            };

            if final_result == windows::Win32::Foundation::NO_ERROR.0 {
                let table_ptr = buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_MODULE;
                let num_entries = unsafe { (*table_ptr).dwNumEntries };
                let row_ptr = unsafe { &((*table_ptr).table) as *const MIB_UDPROW_OWNER_MODULE };

                for i in 0..num_entries {
                    let row = unsafe { &(*row_ptr.add(i as usize)) };
                    match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, row.dwOwningPid) } {
                        Ok(handle) => {
                            let mut path_buffer= [0u16; 260];
                            let length =  unsafe { GetProcessImageFileNameW(handle, &mut path_buffer) };
                            if length > 0 {
                                let path = String::from_utf16_lossy(&path_buffer[..length as usize]);
                                udp_sockets.push(
                                    Socket {
                                        process_name: path.split("\\").last().unwrap_or("unknown").trim().to_string(),
                                        pid: row.dwOwningPid,
                                        port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                        protocol: "UDP",
                                        remote_addr: None,
                                        local_addr: Ipv4Addr::from(row.dwLocalAddr.to_be()).to_string(),
                                        remote_port: None,
                                        state: "-".to_string(),
                                        executable_path: to_dos_path(&path)
                                    }
                                );
                            }
                            unsafe { windows::Win32::Foundation::CloseHandle(handle).ok(); }
                        }
                        Err(_) => {}
                    } 
                }
            } 
        }
    }
    udp_sockets
}

pub fn get_tcp_sockets(ulaf: u32) -> Vec<Socket> {
    let mut tcp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedTcpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            ulaf,
            TCP_TABLE_OWNER_MODULE_ALL,
            0,
        )
    };

    if result != NO_ERROR.0 {
        if result == windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER.0 {
            let mut buffer = vec![0u8; table_size as usize];
            let final_result = unsafe {
                GetExtendedTcpTable(
                    Some(buffer.as_mut_ptr() as *mut c_void),
                    &mut table_size,
                    false,
                    ulaf,
                    TCP_TABLE_OWNER_MODULE_ALL,
                    0,
                )
            };

            if final_result == windows::Win32::Foundation::NO_ERROR.0 {
                let table_ptr = buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_MODULE;
                let num_entries = unsafe { (*table_ptr).dwNumEntries };
                let row_ptr = unsafe { &((*table_ptr).table) as *const MIB_TCPROW_OWNER_MODULE };

                for i in 0..num_entries {
                    let row = unsafe { &(*row_ptr.add(i as usize)) };
                    match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, row.dwOwningPid) } {
                        Ok(handle) => {
                            let mut path_buffer= [0u16; 260];
                            let length = unsafe { GetProcessImageFileNameW(handle, &mut path_buffer) };
                            if length > 0 {
                                let path = String::from_utf16_lossy(&path_buffer[..length as usize]);
                                tcp_sockets.push(
                                    Socket {
                                        process_name: path.split("\\").last().unwrap_or("unknown").trim().to_string(),
                                        pid: row.dwOwningPid,
                                        port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                        protocol: "TCP",
                                        remote_addr: Some(Ipv4Addr::from(row.dwRemoteAddr.to_be()).to_string()),
                                        local_addr: Ipv4Addr::from(row.dwLocalAddr.to_be()).to_string(),
                                        remote_port: Some(u16::from_be((row.dwRemotePort & 0xFFFF) as u16)),
                                        state: map_tcp_state(row.dwState),
                                        executable_path: to_dos_path(&path)
                                    }
                                );
                            }
                            unsafe { windows::Win32::Foundation::CloseHandle(handle).ok(); }
                        }
                        Err(_) => {}
                    } 
                }
            }
        } 
    }
    tcp_sockets
}

//--------------------------------------------------------------------------------------------------------------------------