use std::{ffi::c_void, net::{Ipv4Addr, Ipv6Addr}, ptr};

use windows::Win32::{Foundation::NO_ERROR, NetworkManagement::IpHelper::{GetExtendedTcpTable, MIB_TCP6ROW_OWNER_MODULE, MIB_TCP6TABLE_OWNER_MODULE, MIB_TCPROW_OWNER_MODULE, MIB_TCPTABLE_OWNER_MODULE, TCP_TABLE_OWNER_MODULE_ALL}, System::{ProcessStatus::GetProcessImageFileNameW, Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}}};

use crate::tools::{nt_to_dos::to_dos_path, socket::{map_tcp_state, Socket, IPV4_ULAF, IPV6_ULAF}};

pub fn get_tcp_sockets() -> Vec<Socket> {
    let mut tcp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedTcpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            IPV4_ULAF,
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
                    IPV4_ULAF,
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
                    if row.dwOwningPid == 0 {
                        continue;
                    }
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
                        Err(_) => {
                            tcp_sockets.push(
                                Socket {
                                    process_name: match row.dwOwningPid {
                                        4 => {
                                            "SYSTEM".to_string()
                                        }
                                        _ => {
                                            " ".to_string()
                                        }
                                    },
                                    pid: row.dwOwningPid,
                                    port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                    protocol: "TCP",
                                    remote_addr: Some(Ipv4Addr::from(row.dwRemoteAddr.to_be()).to_string()),
                                    local_addr: Ipv4Addr::from(row.dwLocalAddr.to_be()).to_string(),
                                    remote_port: Some(u16::from_be((row.dwRemotePort & 0xFFFF) as u16)),
                                    state: map_tcp_state(row.dwState),
                                    executable_path: None
                                }
                            );
                        }
                    } 
                }
            }
        } 
    }
    tcp_sockets
}

pub fn get_tcp_sockets_ipv6() -> Vec<Socket> {
    let mut tcp_sockets: Vec<Socket> = Vec::new();

    let mut table_size = 0;
    let result = unsafe {
        GetExtendedTcpTable(
            Some(ptr::null_mut() as *mut c_void),
            &mut table_size,
            false,
            IPV6_ULAF,
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
                    IPV6_ULAF,
                    TCP_TABLE_OWNER_MODULE_ALL,
                    0,
                )
            };

            if final_result == windows::Win32::Foundation::NO_ERROR.0 {
                let table_ptr = buffer.as_ptr() as *const MIB_TCP6TABLE_OWNER_MODULE;
                let num_entries = unsafe { (*table_ptr).dwNumEntries };
                let row_ptr = unsafe { &((*table_ptr).table) as *const MIB_TCP6ROW_OWNER_MODULE };

                for i in 0..num_entries {
                    let row = unsafe { &(*row_ptr.add(i as usize)) };
                    if row.dwOwningPid == 0 {
                        continue;
                    }
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
                                        remote_addr: Some(Ipv6Addr::from(row.ucRemoteAddr).to_string()),
                                        local_addr: Ipv6Addr::from(row.ucLocalAddr).to_string(),
                                        remote_port: Some(u16::from_be((row.dwRemotePort & 0xFFFF) as u16)),
                                        state: map_tcp_state(row.dwState),
                                        executable_path: to_dos_path(&path)
                                    }
                                );
                            }
                            unsafe { windows::Win32::Foundation::CloseHandle(handle).ok(); }
                        }
                        Err(_) => {
                            tcp_sockets.push(
                                Socket {
                                    process_name: match row.dwOwningPid {
                                        4 => {
                                            "SYSTEM".to_string()
                                        }
                                        _ => {
                                            " ".to_string()
                                        }
                                    },
                                    pid: row.dwOwningPid,
                                    port: u16::from_be((row.dwLocalPort & 0xFFFF) as u16),
                                    protocol: "TCP",
                                    remote_addr: Some(Ipv6Addr::from(row.ucRemoteAddr).to_string()),
                                    local_addr: Ipv6Addr::from(row.ucLocalAddr).to_string(),
                                    remote_port: Some(u16::from_be((row.dwRemotePort & 0xFFFF) as u16)),
                                    state: map_tcp_state(row.dwState),
                                    executable_path: None
                                }
                            );
                        }
                    } 
                }
            }
        } 
    }
    tcp_sockets
}
